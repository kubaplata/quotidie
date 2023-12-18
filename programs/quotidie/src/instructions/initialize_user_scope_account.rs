use anchor_lang::prelude::*;
use crate::states::*;

pub fn initialize_user_scope_account(
    ctx: Context<InitializeUserScopeAccount>,
    offset: i8
) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_scope_account = &mut ctx.accounts.user_scope_account;

    user_scope_account.user = user.key();
    user_scope_account.expires = 0;
    user_scope_account.loyalty = 0;
    user_scope_account.next_claim = 0;
    user_scope_account.offset = offset;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(scope_id: i64)]
pub struct InitializeUserScopeAccount<'info> {
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
        init,
        space = 4 + 8,
        seeds = [
            "quotidie_user_scope_account".as_bytes(),
            &scope_id.to_be_bytes(),
            &user.key().as_ref()
        ],
        bump,
        payer = user,
    )]
    pub user_scope_account: Account<'info, UserScopeAccount>,

    #[account()]
    pub system_program: Program<'info, System>
}