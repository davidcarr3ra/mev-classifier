use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct WhirlpoolsSwapAction {
    pub pool: Pubkey,
    pub amount: u64,
}
