use macros::action;
use solana_sdk::signature::Signature;

use super::ActionTrait;

/// Basic action signifying all sub-actions happened in the
/// same transaction
#[action]
pub struct Transaction {
    pub signature: Signature,
}

impl ActionTrait for Transaction {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("Transaction should not be classified")
    }
}

impl Transaction {
    pub fn new(signature: Signature) -> Self {
        Self { signature }
    }
}
