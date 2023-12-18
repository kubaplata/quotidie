use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use crate::states::*;

pub fn initialize_scope(
    ctx: Context<InitializeScope>,
    length: i64,
) -> Result<()> {
    let quotidie = &mut ctx.accounts.quotidie;

    let admin = &mut ctx.accounts.admin;
    let scope = &mut ctx.accounts.scope;

    scope.admin = admin.key();
    scope.id = quotidie.total_scopes;
    scope.bump = *ctx.bumps.get("scope").unwrap();
    scope.length = length;
    scope.emissions = Vec::new();
    scope.total_users = 0;

    quotidie.total_scopes += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(length: i64)]
pub struct InitializeScope<'info> {
    #[account(
        mut
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [
            "quotidie_protocol".as_bytes()
        ],
        bump,
    )]
    pub quotidie: Account<'info, QuotidieProtocol>,

    #[account(
        init,
        seeds = [
            "quotidie_scope".as_bytes(),
            &quotidie.total_scopes.to_be_bytes()
        ],
        bump,
        space = 8 + 8 + 32 + 8 + 8 + 4 + (usize::try_from(length).unwrap() * 16),
        payer = admin,
    )]
    pub scope: Account<'info, Scope>,

    #[account(
        mut,
        constraint = emissions_vault.mint == scope.token.key(),
        constraint = emissions_vault.owner == scope.key(),
    )]
    pub emissions_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub token: Account<'info, Mint>,

    #[account()]
    pub system_program: Program<'info, System>
}