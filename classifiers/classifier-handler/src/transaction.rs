use action_tree::{ActionNodeId, ActionTree};
use classifier_core::ClassifiableTransaction;
use thiserror::Error;

use crate::instruction::{classify_instruction, ClassifyInstructionError};

#[derive(Debug, Error)]
pub enum ClassifyError {
    #[error(transparent)]
    ClassifyInstructionError(#[from] ClassifyInstructionError),
}

type Result<T> = std::result::Result<T, ClassifyError>;

pub fn classify_transaction(
    txn: ClassifiableTransaction,
    tree: &mut ActionTree,
    parent: ActionNodeId,
) -> Result<()> {
    let mut idx = 0;

    while idx < txn.instructions.len() {
        let indexes_used = classify_instruction(&txn, idx, tree, parent)?;
        idx += indexes_used;
    }

    Ok(())
}
