use anchor_lang::prelude::*;
use crate::states::{QuotidieProtocol};

pub fn initialize_protocol(
    ctx: Context<InitializeProtocol>
) -> Result<()> {
    let protocol = &mut ctx.accounts.quotidie;
    let admin = &mut ctx.accounts.admin;
    
    protocol.admin = admin.key();
    protocol.total_scopes = 0;
    protocol.total_users = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(
        mut
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [
            "quotidie_protocol".as_bytes()
        ],
        bump,
        payer = admin,
        space = 8 + (1 * 32) + (2 * 8)
    )]
    pub quotidie: Account<'info, QuotidieProtocol>,

    #[account()]
    pub system_program: Program<'info, System>
}