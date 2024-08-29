use solana_sdk::pubkey::Pubkey;

use super::Action;

#[derive(Clone, Debug)]
pub struct NativeTransferAction {
    pub from: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
}

impl NativeTransferAction {
    pub fn new(from: Pubkey, to: Pubkey, lamports: u64) -> Self {
        Self { from, to, lamports }
    }
}

impl Into<Action> for NativeTransferAction {
    fn into(self) -> Action {
        Action::NativeTransfer(self)
    }
}
