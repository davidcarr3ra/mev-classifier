use solana_sdk::pubkey::Pubkey;

pub struct TokenTransfer {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
