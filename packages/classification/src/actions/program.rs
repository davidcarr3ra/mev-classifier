use macros::action;
use solana_sdk::pubkey::Pubkey;

#[action]
pub struct ProgramInvocation {
    pub program_id: Pubkey,
}
