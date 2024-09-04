use classifier_core::ClassifiableTransaction;
use macros::action;
use solana_sdk::signature::Signature;

use super::ActionTrait;

impl ActionTrait for ClassifiableTransaction {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}

/// Basic action signifying all sub-actions happened in the
/// same transaction
#[action]
pub struct Transaction {
    pub signature: Signature,
}

impl Transaction {
    pub fn new(signature: Signature) -> Self {
        Self { signature }
    }
}

impl ActionTrait for Transaction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
