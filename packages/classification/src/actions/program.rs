use macros::action;
use solana_sdk::pubkey::Pubkey;

use super::ActionTrait;

#[action]
pub struct ProgramInvocation {
    pub program_id: Pubkey,
}

impl ActionTrait for ProgramInvocation {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}
