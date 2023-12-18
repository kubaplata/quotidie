use anchor_lang::prelude::*;

#[account]
pub struct UserScopeAccount {
    pub user: Pubkey,
    pub loyalty: i64,
    pub offset: i8,
    pub expires: i64,
    pub next_claim: i64,
}