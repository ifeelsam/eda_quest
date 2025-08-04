use anchor_lang::prelude::*;
use crate::{GameState, PlayerAccount};
use crate::state::SubscriptionType;

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + PlayerAccount::INIT_SPACE,
        seeds = [b"player", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: This is the authority account that receives payments
    #[account(mut)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Subscribe<'info> {
    pub fn subscribe_monthly(&mut self) -> Result<()> {
        let game_state = &self.game_state;
        let player_account = &mut self.player_account;
        
        // Transfer SOL for monthly subscription
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &self.player.key(),
            &self.authority.key(),
            game_state.monthly_price,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.player.to_account_info(),
                self.authority.to_account_info(),
            ],
        )?;

        // Set subscription details
        let clock = Clock::get()?;
        player_account.player = self.player.key();
        player_account.subscription_type = SubscriptionType::Monthly;
        player_account.subscription_start = clock.unix_timestamp;
        player_account.subscription_end = clock.unix_timestamp + 30 * 24 * 60 * 60; // 30 days
        player_account.game_progress = 0;
        player_account.has_completion_nft = false;

        Ok(())
    }

    pub fn subscribe_yearly(&mut self) -> Result<()> {
        let game_state = &self.game_state;
        let player_account = &mut self.player_account;
        
        // Transfer SOL for yearly subscription
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &self.player.key(),
            &self.authority.key(),
            game_state.yearly_price,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.player.to_account_info(),
                self.authority.to_account_info(),
            ],
        )?;

        // Set subscription details
        let clock = Clock::get()?;
        player_account.player = self.player.key();
        player_account.subscription_type = SubscriptionType::Yearly;
        player_account.subscription_start = clock.unix_timestamp;
        player_account.subscription_end = clock.unix_timestamp + 365 * 24 * 60 * 60; // 365 days
        player_account.game_progress = 0;
        player_account.has_completion_nft = false;

        Ok(())
    }
}