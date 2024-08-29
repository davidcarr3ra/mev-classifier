use solana_sdk::pubkey::Pubkey;

pub struct DexSwapAction {
    pub input: Pubkey,
    pub output: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
}
