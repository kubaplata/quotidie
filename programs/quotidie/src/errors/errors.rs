use anchor_lang::prelude::*;

#[error_code]
pub enum QuotidieError {
    #[msg("Cannot add new emission. Emission period specified at the time of initialization is too short.")]
    EmissionTooShort,

    #[msg("Cannot claim the next emission. Current loyalty level exceeds the number of emissions.")]
    MaxLoyaltyAchieved,

    #[msg("Numerical overflow error")]
    NumericalOverflow,

    #[msg("Failed to transfer tokens. Not enough tokens in the emissions vault.")]
    VaultBalanceTooLow,

    #[msg("Daily loyalty streak expired. Emissions cannot be claimed anymore")]
    LoyaltyExpired
}