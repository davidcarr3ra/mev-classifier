use solana_sdk::pubkey::Pubkey;

#[derive(Debug, PartialEq, Eq)]
pub struct WhirlpoolsSwapAction {
    pub pool: Pubkey,
    pub amount: u64,
}
