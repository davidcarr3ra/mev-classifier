use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct TokenTransfer {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
