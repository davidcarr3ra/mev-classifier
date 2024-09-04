use macros::action;
use solana_sdk::pubkey::Pubkey;

use crate::ActionTrait;

#[action]
pub struct AtomicArbitrage {
    pub mint: Pubkey,
    pub profit_amount: u64,
}

impl AtomicArbitrage {
    pub fn new(mint: Pubkey, profit_amount: u64) -> Self {
        Self {
            mint,
            profit_amount,
        }
    }
}

impl ActionTrait for AtomicArbitrage {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("AtomicArbitrage should not be classified");
    }
}
