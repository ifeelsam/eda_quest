use anchor_lang::prelude::*;

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
        space = 8 + 32 + 1 + 8 + 8 + 1 + 1,
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

impl <'info> Subscribe<'info> {
    
    
}