use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::*;

pub fn add_emission(
    ctx: Context<AddEmission>,
    min: i64,
    max: i64,
) -> Result<()> {

    let scope = &mut ctx.accounts.scope;
    let emissions = &mut scope.emissions;

    require!(
        ((*emissions).len() as i64) < scope.length, 
        QuotidieError::EmissionTooShort
    );

    scope.add_emission(
        min, 
        max
    )
}

#[derive(Accounts)]
pub struct AddEmission<'info> {
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
        mut,
        seeds = [
            "quotidie_scope".as_bytes(),
            &quotidie.total_scopes.to_be_bytes()
        ],
        bump,
        constraint = scope.admin == admin.key()
    )]
    pub scope: Account<'info, Scope>,
}