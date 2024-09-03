use actions::{is_jito_tip_address, Action, JitoTip, NativeTransfer};
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::{pubkey::Pubkey, system_instruction::SystemInstruction};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClassifySystemInstructionError {
    #[error("Failed to deserialize system instruction")]
    DeserializeError(#[from] bincode::Error),

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Missing account")]
    MissingAccount,
}

type Result<T> = std::result::Result<T, ClassifySystemInstructionError>;

pub struct SystemProgramClassifier;

impl InstructionClassifier for SystemProgramClassifier {
    const ID: Pubkey = solana_sdk::system_program::ID;

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let parsed_ix: SystemInstruction = bincode::deserialize(&ix.data)
            .map_err(ClassifySystemInstructionError::DeserializeError)?;

        let action = match parsed_ix {
            SystemInstruction::Transfer { lamports } => classify_transfer(lamports, txn, ix)?,

            _ => return Ok(None),
        };

        Ok(Some(action))
    }
}

/// # Account references
///   0. `[WRITE, SIGNER]` Funding account
///   1. `[WRITE]` Recipient account
fn classify_transfer(
    lamports: u64,
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> Result<Action> {
    if ix.accounts.len() != 2 {
        return Err(ClassifySystemInstructionError::InvalidInstruction);
    }

    let funding = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| ClassifySystemInstructionError::MissingAccount)?;

    let recipient = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| ClassifySystemInstructionError::MissingAccount)?;

    if is_jito_tip_address(&recipient) {
        Ok(JitoTip::new(funding, lamports).into())
    } else {
        Ok(NativeTransfer::new(funding, recipient, lamports).into())
    }
}
