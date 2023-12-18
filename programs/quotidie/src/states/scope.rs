use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DailyEmission {
    pub min: i64, // 8
    pub max: i64, // 8
}

#[account]
pub struct Scope {
    pub id: i64, // 8
    pub admin: Pubkey, // 32
    pub total_users: i64, // 8
    pub length: i64, // 8
    pub emissions: Vec<DailyEmission>, // 4 + (length * sizeof(DailyEmission -> 16))
    pub token: Pubkey,
    pub emissions_vault: Pubkey,
    pub bump: u8,
}


impl Scope {
    pub fn add_emission(&mut self, min: i64, max: i64) -> Result<()> {
        self.emissions.push(DailyEmission{
            min: min,
            max: max
        });

        Ok(())
    }
}