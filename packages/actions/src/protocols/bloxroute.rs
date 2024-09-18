use crate::ActionTrait;
use macros::action;
use solana_sdk::pubkey::Pubkey;

// https://docs.bloxroute.com/solana/trader-api-v2/front-running-protection-and-transaction-bundle
pub const BLOXROUTE_TIP_ADDRESS: Pubkey =
    solana_sdk::pubkey!("HWEoBxYs7ssKuudEjzjmpfJVX7Dvi7wescFsVx2L5yoY");

pub fn is_bloxroute_tip_address(address: &Pubkey) -> bool {
    *address == BLOXROUTE_TIP_ADDRESS
}

#[action]
pub struct BloxrouteTip {
    pub tipper: Pubkey,
    pub tip_amount: u64,
}

impl BloxrouteTip {
    pub fn new(tipper: Pubkey, tip_amount: u64) -> Self {
        Self { tipper, tip_amount }
    }
}

impl ActionTrait for BloxrouteTip {
    fn recurse_during_classify(&self) -> bool {
        false
    }

    fn serializable(&self) -> bool {
        true
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "bloxrouteTip",
            "tipper": self.tipper.to_string(),
            "tipAmount": self.tip_amount,
        })
    }
}
