use actions::{Action, SetComputeBudgetLimit, SetComputeUnitPrice};
use borsh::BorshDeserialize;
use classifier_core::{
    ClassifiableInstruction, ClassifiableTransaction, ClassifyInstructionResult,
    InstructionClassifier,
};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, pubkey::Pubkey};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClassifyComputeBudgetError {
    #[error("Failed to deserialize")]
    DeserializeError(#[from] borsh::io::Error),
}

pub struct ComputeBudgetClassifier;

impl InstructionClassifier for ComputeBudgetClassifier {
    const ID: Pubkey = solana_sdk::compute_budget::ID;

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let decoded: ComputeBudgetInstruction =
            ComputeBudgetInstruction::deserialize(&mut &ix.data[..])?;

        let action = match decoded {
            ComputeBudgetInstruction::SetComputeUnitLimit(units) => {
                Some(Action::SetComputeBudgetLimit(SetComputeBudgetLimit {
                    units,
                }))
            }
            ComputeBudgetInstruction::SetComputeUnitPrice(micro_lamports) => {
                Some(Action::SetComputeUnitPrice(SetComputeUnitPrice {
                    micro_lamports,
                }))
            }
            ComputeBudgetInstruction::RequestHeapFrame(_) => None,
            ComputeBudgetInstruction::SetLoadedAccountsDataSizeLimit(_) => None,
            ComputeBudgetInstruction::Unused => None,
        };

        Ok(action)
    }
}
