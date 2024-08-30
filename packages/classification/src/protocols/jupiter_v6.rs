use anchor_lang::{declare_program, AnchorDeserialize, Discriminator};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

use crate::{
    transaction::ClassifiableInstruction, Action, ClassifiableTransaction,
    JupiterV6LedgerSwapAction, JupiterV6SwapAction,
};

declare_program!(jupiter_v6);

#[derive(Debug, Error)]
pub enum JupiterV6Error {
    #[error("Invalid instruction data length")]
    InvalidLength,

    #[error("Failed to deserialize jupiter v6 instruction")]
    DeserializationError,
}

type Result<T> = std::result::Result<T, JupiterV6Error>;

pub const ID: Pubkey = jupiter_v6::ID_CONST;

pub fn classify_instruction(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> Result<Option<Action>> {
    if ix.data.len() < 8 {
        return Err(JupiterV6Error::InvalidLength);
    }

    let discriminator = &ix.data[..8];

    let action = match discriminator {
        jupiter_v6::internal::args::Route::DISCRIMINATOR => classify_route(txn, ix)?,
        jupiter_v6::internal::args::SharedAccountsRoute::DISCRIMINATOR => {
            classify_shared_accounts_route(txn, ix)?
        }

        jupiter_v6::internal::args::RouteWithTokenLedger::DISCRIMINATOR => {
            classify_token_ledger_route(txn, ix)?
        }

        _ => return Ok(None),
    };

    Ok(Some(action))
}

fn classify_route(_txn: &ClassifiableTransaction, ix: &ClassifiableInstruction) -> Result<Action> {
    let mut data = &ix.data[8..];

    let args = jupiter_v6::internal::args::Route::deserialize(&mut data)
        .map_err(|_| JupiterV6Error::DeserializationError)?;

    let action = JupiterV6SwapAction {
        in_amount: args.in_amount,
        quoted_out_amount: args.quoted_out_amount,
    };

    Ok(action.into())
}

fn classify_shared_accounts_route(
    _txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> Result<Action> {
    let mut data = &ix.data[8..];

    let args = jupiter_v6::internal::args::SharedAccountsRoute::deserialize(&mut data)
        .map_err(|_| JupiterV6Error::DeserializationError)?;

    let action = JupiterV6SwapAction {
        in_amount: args.in_amount,
        quoted_out_amount: args.quoted_out_amount,
    };

    Ok(action.into())
}

fn classify_token_ledger_route(
    _txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> Result<Action> {
    let mut data = &ix.data[8..];

    let args = jupiter_v6::internal::args::RouteWithTokenLedger::deserialize(&mut data)
        .map_err(|_| JupiterV6Error::DeserializationError)?;

    let action = JupiterV6LedgerSwapAction {
        quoted_out_amount: args.quoted_out_amount,
    };

    Ok(action.into())
}
