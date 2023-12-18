use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

pub mod instructions;
pub use instructions::*;

pub mod errors;
pub use errors::*;

pub mod common;
pub use common::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod quotidie {
    use super::*;

    pub fn initialize_protocol(ctx: Context<InitializeProtocol>) -> Result<()> {
        instructions::initialize_protocol(
            ctx
        )
    }
}
