use solana_sdk::system_instruction::SystemInstruction;
use thiserror::Error;

use crate::{
    actions::{is_jito_tip_address, JitoTipAction, NativeTransferAction},
    transaction::ClassifiableInstruction,
    Action, ClassifiableTransaction,
};

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

pub fn classify_instruction(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> Result<Option<Action>> {
    let parsed_ix: SystemInstruction = match bincode::deserialize(&ix.data) {
        Ok(ix) => ix,
        Err(err) => {
            return Err(ClassifySystemInstructionError::DeserializeError(err));
        }
    };

    let action = match parsed_ix {
        SystemInstruction::Transfer { lamports } => classify_transfer(lamports, txn, ix)?,

        _ => return Ok(None),
    };

    Ok(Some(action))
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
        Ok(JitoTipAction::new(funding, lamports).into())
    } else {
        Ok(NativeTransferAction::new(funding, recipient, lamports).into())
    }
}
