use actions::{ActionTrait, ProgramInvocation};
use classifier_core::{
    ActionNodeId, ActionTree, ClassifiableInstruction, ClassifiableTransaction,
    InstructionClassifier,
};
use thiserror::Error;

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

/// Utility macro to match instruction to its correct classifier
macro_rules! classify_instruction_matcher {
    ($program_id:expr, $txn:expr, $ix:expr, $($classifier:ty),* $(,)?) => {
        match $program_id {
            $(
                <$classifier>::ID => <$classifier>::classify_instruction($txn, $ix)
                    .map_err(|err| {
                        let classifier = stringify!($classifier);
                        ClassifyInstructionError::ClassificationError(Into::<anyhow::Error>::into(err)
                            .context(format!("Classifier: {}", classifier)))
                    }),
            )*
            _ => Err(ClassifyInstructionError::UnknownProgramId),
        }
    };
}

/// Classifies an instruction, recursing into its inner instructions if
/// necessary.
pub fn classify_instruction(
    txn: &ClassifiableTransaction,
    mut index: usize,
    tree: &mut ActionTree,
    parent: ActionNodeId,
) -> Result<usize> {
    let ix: &ClassifiableInstruction = &txn.instructions[index];
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

        // Solana classifiers
        solana_classifier::ComputeBudgetClassifier,
        solana_classifier::VoteClassifier,
        solana_classifier::SystemProgramClassifier,
        // Third party classifiers
        anchor_classifiers::JupiterV6Classifier,
        anchor_classifiers::OrcaWhirlpoolsClassifier,
    );

    let action = match action_result {
        Ok(action) => action,

        // Still want to classify unknown programs
        Err(ClassifyInstructionError::UnknownProgramId) => {
            Some(ProgramInvocation { program_id }.into())
        }

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
