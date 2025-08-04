use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use crate::{PlayerAccount, GameError};

#[derive(Accounts)]
pub struct MintCompletionNft<'info> {
    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump,
        constraint = player_account.player == player.key()
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(
        init,
        payer = player,
        mint::decimals = 0,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"nft_mint", player.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = player,
        associated_token::mint = mint,
        associated_token::authority = player
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: This is the metadata account
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: This is the authority account
    pub authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: This is the metadata program
    pub token_metadata_program: AccountInfo<'info>,
}

impl<'info> MintCompletionNft<'info> {
    pub fn mint_completion_nft(&mut self, metadata_string: String) -> Result<()> {
        let player_account = &mut self.player_account;
        let clock = Clock::get()?;

        // Check if subscription is active
        require!(
            clock.unix_timestamp <= player_account.subscription_end,
            GameError::SubscriptionExpired
        );

        // Check if game is completed
        require!(
            player_account.game_progress == 100,
            GameError::GameNotCompleted
        );

        // Check if NFT already minted
        require!(
            !player_account.has_completion_nft,
            GameError::NftAlreadyMinted
        );

        // Mint NFT to player
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        mint_to(cpi_ctx, 1)?;

        // Create metadata for the NFT
        let data_v2 = DataV2 {
            name: "Solana Master Graduate".to_string(),
            symbol: "SMG".to_string(),
            uri: metadata_string,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.mint.to_account_info(),
            mint_authority: self.mint.to_account_info(),
            update_authority: self.authority.to_account_info(),
            payer: self.player.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        create_metadata_accounts_v3(cpi_ctx, data_v2, true, true, None)?;

        // Update player account
        player_account.has_completion_nft = true;

        Ok(())
    }
}
