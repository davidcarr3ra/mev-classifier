use thiserror::Error;

use crate::{ActionNodeId, ActionTree, ClassifiableTransaction};

mod orca_whirlpools;
mod system_program;

#[derive(Debug, Error)]
pub enum ClassifyInstructionError {
    #[error("Missing program id")]
    MissingProgramId,

    #[error(transparent)]
    ClassificationError(#[from] anyhow::Error),
}

type Result<T> = std::result::Result<T, ClassifyInstructionError>;

pub fn classify_instruction(
    txn: &ClassifiableTransaction,
    index: usize,
    tree: &mut ActionTree,
    parent: ActionNodeId,
) -> Result<()> {
    let ix: &crate::transaction::ClassifiableInstruction = &txn.instructions[index];
    let program_id = txn
        .get_pubkey(ix.program_id_index)
        .ok_or_else(|| ClassifyInstructionError::MissingProgramId)?;

    let action = match program_id {
        solana_sdk::system_program::ID => system_program::classify_instruction(txn, ix)
            .map_err(|err| ClassifyInstructionError::ClassificationError(err.into())),

        _ => Ok(None),
    }?;

    if let Some(action) = action {
        let recurse = action.recurse_during_classify();
        let child = tree.insert(parent, action);

        if recurse && index + 1 < txn.instructions.len() {
            return classify_instruction(txn, index + 1, tree, child);
        }
    }

    Ok(())
}
