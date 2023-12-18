use anchor_lang::prelude::*;

#[account]
pub struct QuotidieProtocol {
    pub admin: Pubkey, // 32
    pub total_scopes: i64, // 8
    pub total_users: i64, // 8
}