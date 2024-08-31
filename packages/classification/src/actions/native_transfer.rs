use crate::ActionTrait;
use macros::action;
use solana_sdk::pubkey::Pubkey;

#[action]
pub struct NativeTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
}

impl NativeTransfer {
    pub fn new(from: Pubkey, to: Pubkey, lamports: u64) -> Self {
        Self { from, to, lamports }
    }
}

impl ActionTrait for NativeTransfer {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
