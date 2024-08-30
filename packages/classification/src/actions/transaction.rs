use solana_sdk::signature::Signature;

use super::Action;

/// Basic action signifying all sub-actions happened in the
/// same transaction
#[derive(Debug, PartialEq, Eq)]
pub struct TransactionAction {
    pub signature: Signature,
}

impl TransactionAction {
    pub fn new(signature: Signature) -> Self {
        Self { signature }
    }
}

impl From<TransactionAction> for Action {
    fn from(action: TransactionAction) -> Self {
        Action::Transaction(action)
    }
}
