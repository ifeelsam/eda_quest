use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Subscription has expired")]
    SubscriptionExpired,
    #[msg("Invalid progress value. Must be between 0 and 100")]
    InvalidProgress,
    #[msg("Game must be 100% completed to mint NFT")]
    GameNotCompleted,
    #[msg("Completion NFT already minted for this player")]
    NftAlreadyMinted,
}
