use actions::Action;
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use solana_sdk::pubkey::Pubkey;

pub type ClassifyInstructionError = anyhow::Error;
pub type ClassifyInstructionResult = Result<Option<Action>, ClassifyInstructionError>;

pub trait InstructionClassifier {
    /// Used to identify which instruction classifier to use
    const ID: Pubkey;

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult;
}
