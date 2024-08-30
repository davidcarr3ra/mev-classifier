use thiserror::Error;

use crate::{
    protocols::{classify_instruction, ClassifyInstructionError},
    transaction::ClassifiableTransaction,
    tree::{ActionNodeId, ActionTree},
};

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
