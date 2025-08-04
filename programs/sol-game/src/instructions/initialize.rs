use anchor_lang::prelude::*;

use crate::GameState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        self.game_state.set_inner(GameState {
            authority,
            total_players: 0,
            monthly_price: 60_606_060, // ~$10  
            yearly_price: 606_060_606, // ~$100 

        });
        Ok(())
    }
}