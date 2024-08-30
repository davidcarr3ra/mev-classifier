use solana_sdk::{pubkey::Pubkey, vote::state::VoteStateUpdate};

use super::Action;

#[derive(Debug)]
pub enum VoteAction {
    CompactUpdateVoteState(CompactUpdateVoteStateAction),
}

impl VoteAction {
    pub(crate) fn recurse_during_classify(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct CompactUpdateVoteStateAction {
    pub vote_authority: Pubkey,
    pub update: VoteStateUpdate,
}

impl Into<Action> for CompactUpdateVoteStateAction {
    fn into(self) -> Action {
        Action::Vote(VoteAction::CompactUpdateVoteState(self))
    }
}
