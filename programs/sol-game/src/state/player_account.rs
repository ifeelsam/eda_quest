use anchor_lang::prelude::*;

pub struct PlayerAccount {
    pub player: Pubkey,
    pub subscription_type: SubscriptionType,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub game_progress: u8, // 0-100
    pub has_completion_nft: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SubscriptionType {
    Monthly,
    Yearly,
}
