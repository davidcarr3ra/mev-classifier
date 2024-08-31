use thiserror::Error;

use crate::{actions::ActionTrait, ActionNodeId, ActionTree, ClassifiableTransaction};

mod jupiter_v6;
mod orca_whirlpools;
mod system_program;
mod vote_program;

#[derive(Debug, Error)]
pub enum ClassifyInstructionError {
    #[error("Missing program id")]
    MissingProgramId,

    #[error("Unknown program id")]
    UnknownProgramId,

    #[error(transparent)]
    ClassificationError(#[from] anyhow::Error),
}

type Result<T> = std::result::Result<T, ClassifyInstructionError>;

macro_rules! classify_instruction_matcher {
    ($program_id:expr, $txn:expr, $ix:expr, $($mod:ident),* $(,)?) => {
        match $program_id {
            $(
                $mod::ID => $mod::classify_instruction($txn, $ix)
                    .map_err(|err| ClassifyInstructionError::ClassificationError(err.into())),
            )*
            _ => Err(ClassifyInstructionError::UnknownProgramId),
        }
    };
}

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

    let action_result = classify_instruction_matcher!(
        program_id,
        txn,
        ix,
        //
        // Register all classifier modules here
        //
        system_program,
        vote_program,
        orca_whirlpools,
        jupiter_v6
    );

    let action = match action_result {
        Ok(action) => action,

        // Still want to classify unknown programs
        Err(ClassifyInstructionError::UnknownProgramId) => None,

        // All other errors indicate some sort of actual failure
        Err(err) => return Err(err),
    };

    let (recurse, child) = if let Some(action) = action {
        let recurse = action.recurse_during_classify();
        let child = tree.insert(parent, action);
        (recurse, Some(child))
    } else {
        (false, None)
    };

    let current_stack_height = ix.stack_height;
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
