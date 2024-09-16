use crate::ActionTrait;
use macros::action;
use solana_sdk::pubkey::Pubkey;

/// General catch-all action for invoking any program.
/// Used when no specific classifier is available for a program.
#[action]
pub struct ProgramInvocation {
    pub program_id: Pubkey,
}

impl ActionTrait for ProgramInvocation {
    fn recurse_during_classify(&self) -> bool {
        true
    }

    fn serializable(&self) -> bool {
        true
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "program_invocation",
            "program_id": self.program_id.to_string(),
        })
    }
}

#[action]
pub struct NativeTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
}

impl NativeTransfer {
    pub fn new(from: Pubkey, to: Pubkey, lamports: u64) -> Self {
        Self { from, to, lamports }
    }
}

impl ActionTrait for NativeTransfer {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
