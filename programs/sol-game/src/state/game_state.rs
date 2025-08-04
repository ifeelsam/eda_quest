use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub total_players: u64,
    pub monthly_price: u64,
    pub yearly_price: u64,
}
