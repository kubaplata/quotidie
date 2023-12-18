use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use solana_program::{sysvar};
use crate::errors::*;
use crate::states::*;
use crate::common::*;
use arrayref::array_ref;

pub fn claim_emission(
    ctx: Context<ClaimEmission>
) -> Result<()> {
    let user_scope_account = &mut ctx.accounts.user_scope_account;
    let user_ata = &mut ctx.accounts.user_ata;
    let emissions_vault = &mut ctx.accounts.emissions_vault;
    let token_program = &ctx.accounts.token_program;

    let utc_time = Clock::get().unwrap().unix_timestamp;
    let utc_next_day = get_utc_next_day(utc_time);
    let offset = user_scope_account.offset;

    let current_loyalty = user_scope_account.loyalty;

    let scope = &ctx.accounts.scope;
    let scope_id = &scope.id;
    let emissions = &scope.emissions;

    require!(
        user_scope_account.expires < utc_time,
        QuotidieError::LoyaltyExpired
    );
    
    require!(
        (emissions.len() as i64) > current_loyalty,
        QuotidieError::MaxLoyaltyAchieved
    );

    // Pray it fits
    let next_emission = &emissions[(current_loyalty + 1) as usize];
    let min = next_emission.min;
    let max = next_emission.max;

    let recent_slothashes = &mut ctx.accounts.recent_slothashes;
    let slothashes_data = recent_slothashes.data.borrow();
    let pre_seed = array_ref![slothashes_data, 12, 8];

    let seed = i64::from_le_bytes(*pre_seed).saturating_sub(utc_time);

    let next_day_with_offset = get_next_day_with_offset(
        utc_time, 
        utc_next_day,
        offset
    );

    let expires = next_day_with_offset + DAY;
    user_scope_account.loyalty += 1;
    user_scope_account.next_claim = next_day_with_offset;
    user_scope_account.expires = expires;

    calc_and_send_emission(
        min,
        max,
        seed,
        user_ata,
        emissions_vault,
        &scope,
        *scope_id,
        token_program
    )
}

#[derive(Accounts)]
#[instruction(scope_id: i64)]
pub struct ClaimEmission<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            "quotidie_protocol".as_bytes()
        ],
        bump,
    )]
    pub quotidie: Account<'info, QuotidieProtocol>,

    #[account(
        mut,
        seeds = [
            "quotidie_scope".as_bytes(),
            &scope_id.to_be_bytes()
        ],
        bump,
    )]
    pub scope: Account<'info, Scope>,

    #[account(
        mut,
        seeds = [
            "quotidie_user_scope_account".as_bytes(),
            &scope_id.to_be_bytes(),
            &user.key().as_ref()
        ],
        bump,
    )]
    pub user_scope_account: Account<'info, UserScopeAccount>,

    #[account(
        mut,
        constraint = user_ata.mint == scope.token.key(),
        constraint = user_ata.owner == user.key(),
    )]
    pub user_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = emissions_vault.mint == scope.token.key(),
        constraint = emissions_vault.owner == scope.key(),
        constraint = emissions_vault.key() == scope.emissions_vault,
    )]
    pub emissions_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: address determined in account trait
    #[account(address = sysvar::slot_hashes::id())]
    recent_slothashes: AccountInfo<'info>, 

    pub token_program: Program<'info, Token>,
}