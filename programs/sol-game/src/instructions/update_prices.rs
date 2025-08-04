use anchor_lang::prelude::*;
use crate::GameState;

#[derive(Accounts)]
pub struct UpdatePrices<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump,
        constraint = game_state.authority == authority.key()
    )]
    pub game_state: Account<'info, GameState>,
    pub authority: Signer<'info>,
}

impl<'info> UpdatePrices<'info> {
    pub fn update_prices(&mut self, new_monthly_price: u64, new_yearly_price: u64) -> Result<()> {
        let game_state = &mut self.game_state;
        game_state.monthly_price = new_monthly_price;
        game_state.yearly_price = new_yearly_price;
        Ok(())
    }
} 