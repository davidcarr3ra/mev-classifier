use solana_sdk::pubkey::Pubkey;

use super::Action;

#[derive(Debug, PartialEq, Eq)]
pub struct ProgramInvocationAction {
    pub program_id: Pubkey,
}

impl Into<Action> for ProgramInvocationAction {
    fn into(self) -> Action {
        Action::ProgramInvocation(self)
    }
}
