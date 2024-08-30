use thiserror::Error;

use crate::{ActionNodeId, ActionTree, ClassifiableTransaction, ProgramInvocationAction};

mod jupiter_v6;
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
    mut index: usize,
    tree: &mut ActionTree,
    parent: ActionNodeId,
) -> Result<usize> {
    let ix: &crate::transaction::ClassifiableInstruction = &txn.instructions[index];
    let mut indexes_used = 1;

    let program_id = txn
        .get_pubkey(ix.program_id_index)
        .ok_or_else(|| ClassifyInstructionError::MissingProgramId)?;

    let action = match program_id {
        solana_sdk::system_program::ID => system_program::classify_instruction(txn, ix)
            .map_err(|err| ClassifyInstructionError::ClassificationError(err.into())),
        orca_whirlpools::ID => orca_whirlpools::classify_instruction(txn, ix)
            .map_err(|err| ClassifyInstructionError::ClassificationError(err.into())),
        jupiter_v6::ID => jupiter_v6::classify_instruction(txn, ix)
            .map_err(|err| ClassifyInstructionError::ClassificationError(err.into())),

        _ => Ok(Some(ProgramInvocationAction { program_id }.into())),
    }?;

    let (recurse, child) = if let Some(action) = action {
        let recurse = action.recurse_during_classify();
        let child = tree.insert(parent, action);
        (recurse, Some(child))
    } else {
        (false, None)
    };

    let current_stack_height = ix.stack_height;
    println!(
        "index: {} current_stack_height: {:?}",
        index, current_stack_height
    );
    index += 1;

    while index < txn.instructions.len() {
        let next_ix = &txn.instructions[index];
        if next_ix.stack_height <= current_stack_height {
            break;
        }

        if recurse {
            let r_indexes_used = classify_instruction(txn, index, tree, child.unwrap())?;
            index += r_indexes_used;
            indexes_used += r_indexes_used;
        } else {
            index += 1;
            indexes_used += 1;
        }
    }

    Ok(indexes_used)
}
