use actions::{Action, CompactUpdateVoteState, Vote};
use classifier_core::{
    ClassifiableInstruction, ClassifiableTransaction, ClassifyInstructionResult,
    InstructionClassifier,
};
use solana_sdk::{
    pubkey::Pubkey,
    vote::{instruction::VoteInstruction, state::VoteStateUpdate},
};
use thiserror::Error;

pub struct VoteClassifier;

impl InstructionClassifier for VoteClassifier {
    const ID: Pubkey = solana_sdk::vote::program::ID;

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let decoded: VoteInstruction = bincode::deserialize(&ix.data)?;

        let action = match decoded {
            VoteInstruction::CompactUpdateVoteState(update) => {
                Some(classify_compact_update_vote_state(txn, ix, update)?)
            }

            _ => return Ok(None),
        };

        Ok(action)
    }
}

#[derive(Debug, Error)]
pub enum ClassifyVoteError {
    #[error("Deserialization error")]
    DeserializationError(#[from] bincode::Error),

    #[error("Missing accounts")]
    MissingAccounts,
}

type Result<T> = std::result::Result<T, ClassifyVoteError>;

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
