use actions::{Action, DexSwap, WhirlpoolsSwapAction};
use anchor_lang::{declare_program, AnchorDeserialize, Discriminator};
use classifier_core::{
    ClassifiableInstruction, ClassifiableTransaction, ClassifyInstructionResult,
    InstructionClassifier,
};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

declare_program!(whirlpools);

#[derive(Debug, Error)]
pub enum ClassifyWhirlpoolError {
    #[error("Invalid instruction data length")]
    InvalidLength,

    #[error("Failed to deserialize whirlpool instruction")]
    DeserializationError,

    #[error("Missing accounts")]
    MissingAccounts,
}

type Result<T> = std::result::Result<T, ClassifyWhirlpoolError>;

pub struct OrcaWhirlpoolsClassifier;

impl InstructionClassifier for OrcaWhirlpoolsClassifier {
    const ID: Pubkey = whirlpools::ID_CONST;

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        if ix.data.len() < 8 {
            return Err(ClassifyWhirlpoolError::InvalidLength.into());
        }

        let discriminator = &ix.data[..8];

        let action = match discriminator {
            whirlpools::internal::args::Swap::DISCRIMINATOR => classify_swap(txn, ix)?,
            whirlpools::internal::args::SwapV2::DISCRIMINATOR => classify_swap_v2(txn, ix)?,
            _ => return Ok(None),
        };

        Ok(Some(action))
    }
}

fn classify_swap(txn: &ClassifiableTransaction, ix: &ClassifiableInstruction) -> Result<Action> {
    let mut data = &ix.data[8..];

    let args = whirlpools::internal::args::Swap::deserialize(&mut data)
        .map_err(|_| ClassifyWhirlpoolError::DeserializationError)?;

    let whirlpool = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| ClassifyWhirlpoolError::MissingAccounts)?;

    let action = WhirlpoolsSwapAction {
        pool: whirlpool,
        amount: args.amount,
    };

    Ok(Action::DexSwap(DexSwap::Whirlpools(action)))
}

fn classify_swap_v2(txn: &ClassifiableTransaction, ix: &ClassifiableInstruction) -> Result<Action> {
    let mut data = &ix.data[8..];

    let args = whirlpools::internal::args::SwapV2::deserialize(&mut data)
        .map_err(|_| ClassifyWhirlpoolError::DeserializationError)?;

    let whirlpool = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| ClassifyWhirlpoolError::MissingAccounts)?;

    let action = WhirlpoolsSwapAction {
        pool: whirlpool,
        amount: args.amount,
    };

    Ok(Action::DexSwap(DexSwap::Whirlpools(action)))
}
