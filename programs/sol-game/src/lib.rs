pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("91juefdypagioebacMPA6w1jVqUv7fPRpqqm7D4pM2RW");

// SubscriptionType is now defined in state/player_account.rs

#[program]
pub mod sol_game {
    use super::*;

    // Initialize the game program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }

    // Subscribe monthly
    pub fn subscribe_monthly(ctx: Context<Subscribe>) -> Result<()> {
        ctx.accounts.subscribe_monthly()
    }

    // Subscribe yearly
    pub fn subscribe_yearly(ctx: Context<Subscribe>) -> Result<()> {
        ctx.accounts.subscribe_yearly()
    }

    // Update game progress
    pub fn update_progress(ctx: Context<UpdateProgress>, new_progress: u8) -> Result<()> {
        ctx.accounts.update_progress(new_progress)
    }

    // Temporarily commented out for dependency resolution
    // pub fn mint_completion_nft(
    //     ctx: Context<MintCompletionNft>,
    //     metadata_string: String,
    // ) -> Result<()> {
    //     ctx.accounts.mint_completion_nft(metadata_string)
    // }

    // Admin function to update subscription prices
    pub fn update_prices(
        ctx: Context<UpdatePrices>,
        new_monthly_price: u64,
        new_yearly_price: u64,
    ) -> Result<()> {
        ctx.accounts.update_prices(new_monthly_price, new_yearly_price)
    }
}
