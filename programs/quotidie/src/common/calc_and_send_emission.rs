use anchor_lang::prelude::*;
use crate::errors::*;
use crate::states::*;
use anchor_spl::token::{ self, Token, TokenAccount, Transfer };

pub fn calc_and_send_emission<'info>(
    min: i64,
    max: i64,
    seed: i64,
    user_ata: &Box<Account<'info, TokenAccount>>,
    vault_ata: &Box<Account<'info, TokenAccount>>,
    scope: &Account<'info, Scope>,
    scope_id: i64,
    token_program: &Program<'info, Token>
) -> Result<()> {
    let remainder = seed
        .checked_rem((max - min) as i64)
        .ok_or(QuotidieError::NumericalOverflow)?;
    
    let emission = min + remainder;

    require!(
        vault_ata.amount > (emission as u64),
        QuotidieError::VaultBalanceTooLow
    );

    let seeds = &[
        "quotidie_scope".as_bytes(),
        &scope_id.to_be_bytes(),
        &[scope.bump],
    ];

    // Seeds for PDA transfer
    let signer = &[&seeds[..]];
    let cpi_accounts = Transfer {
        from: vault_ata.to_account_info(),
        to: user_ata.to_account_info(),
        authority: scope.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info().clone(),
            cpi_accounts,
            signer
        ),
        emission.try_into().unwrap()
    )?;

    Ok(())
}