use anchor_lang::prelude::*;
use crate::{PlayerAccount, GameError};

#[derive(Accounts)]
pub struct UpdateProgress<'info> {
    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump,
        constraint = player_account.player == player.key()
    )]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
}

impl<'info> UpdateProgress<'info> {
    pub fn update_progress(&mut self, new_progress: u8) -> Result<()> {
        let player_account = &mut self.player_account;
        let clock = Clock::get()?;

        // Check if subscription is still active
        require!(
            clock.unix_timestamp <= player_account.subscription_end,
            GameError::SubscriptionExpired
        );

        // Update progress (0-100)
        require!(new_progress <= 100, GameError::InvalidProgress);
        player_account.game_progress = new_progress;

        Ok(())
    }
} 