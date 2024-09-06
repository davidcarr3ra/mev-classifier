use macros::{action, action_enum};
use solana_sdk::{pubkey::Pubkey, vote::state::VoteStateUpdate};

use crate::ActionTrait;

#[action_enum]
pub enum Vote {
    CompactUpdateVoteState(CompactUpdateVoteState),
}

impl Vote {
    pub(crate) fn recurse_during_classify(&self) -> bool {
        false
    }
}

impl ActionTrait for Vote {
    fn recurse_during_classify(&self) -> bool {
        self.recurse_during_classify()
    }
}

#[action]
pub struct CompactUpdateVoteState {
    pub vote_authority: Pubkey,
    pub update: VoteStateUpdate,
}

impl Into<Vote> for CompactUpdateVoteState {
    fn into(self) -> Vote {
        Vote::CompactUpdateVoteState(self)
    }
}
