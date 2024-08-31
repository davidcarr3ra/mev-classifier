use solana_sdk::{
    pubkey::Pubkey,
    vote::{instruction::VoteInstruction, state::VoteStateUpdate},
};
use thiserror::Error;

use crate::{
    transaction::ClassifiableInstruction, Action, ClassifiableTransaction, CompactUpdateVoteState,
    Vote,
};

pub const ID: Pubkey = solana_sdk::vote::program::ID;

#[derive(Debug, Error)]
pub enum ClassifyVoteError {
    #[error("Deserialization error")]
    DeserializationError(#[from] bincode::Error),

    #[error("Missing accounts")]
    MissingAccounts,
}

type Result<T> = std::result::Result<T, ClassifyVoteError>;

pub fn classify_instruction(
    txn: &ClassifiableTransaction,
    ixn: &ClassifiableInstruction,
) -> Result<Option<Action>> {
    let instruction: VoteInstruction = bincode::deserialize(&ixn.data)?;

    let action = match instruction {
        VoteInstruction::CompactUpdateVoteState(update) => {
            classify_compact_update_vote_state(txn, ixn, update)?
        }

        _ => return Ok(None),
    };

    Ok(Some(action))
}

fn classify_compact_update_vote_state(
    txn: &ClassifiableTransaction,
    ixn: &ClassifiableInstruction,
    update: VoteStateUpdate,
) -> Result<Action> {
    if ixn.accounts.len() < 1 {
        return Err(ClassifyVoteError::MissingAccounts);
    }

    let vote_authority = txn
        .get_pubkey(ixn.accounts[0])
        .ok_or_else(|| ClassifyVoteError::MissingAccounts)?;

    let vote_action: Vote = CompactUpdateVoteState {
        vote_authority,
        update,
    }
    .into();

    Ok(vote_action.into())
}
