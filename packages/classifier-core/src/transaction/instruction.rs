use solana_sdk::instruction::CompiledInstruction;
use solana_transaction_status::{UiCompiledInstruction, UiInstruction};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClassifiableInstructionError {
    #[error("Missing stack height")]
    MissingStackHeight,

    #[error("Unsupported data format")]
    Unsupported,

    #[error(transparent)]
    DecodeBase58Error(#[from] bs58::decode::Error),
}

type Result<T> = std::result::Result<T, ClassifiableInstructionError>;

#[derive(Debug, Clone)]
pub struct ClassifiableInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub stack_height: u32,
}

impl ClassifiableInstruction {
    pub fn from_compiled(ix: CompiledInstruction, stack_height: u32) -> Self {
        Self {
            program_id_index: ix.program_id_index,
            accounts: ix.accounts,
            data: ix.data,
            stack_height,
        }
    }

    pub fn from_ui(ix: UiInstruction) -> Result<Self> {
        match ix {
            UiInstruction::Parsed(_) => return Err(ClassifiableInstructionError::Unsupported),
            UiInstruction::Compiled(ix) => Self::from_ui_compiled(ix),
        }
    }

    pub fn from_ui_compiled(ix: UiCompiledInstruction) -> Result<Self> {
        let stack_height = ix
            .stack_height
            .ok_or_else(|| ClassifiableInstructionError::MissingStackHeight)?;

        // TODO: Support different encoding formats
        let data = bs58::decode(ix.data).into_vec()?;

        Ok(Self {
            program_id_index: ix.program_id_index,
            accounts: ix.accounts,
            data,
            stack_height,
        })
    }
}
