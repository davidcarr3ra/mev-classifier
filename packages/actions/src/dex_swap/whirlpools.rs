use macros::action;
use solana_sdk::pubkey::Pubkey;

#[action]
pub struct WhirlpoolsSwapAction {
    pub pool: Pubkey,
    pub amount: u64,
}
