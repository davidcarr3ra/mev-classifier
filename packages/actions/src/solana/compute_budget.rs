use macros::action;

use crate::ActionTrait;

#[action]
pub struct SetComputeBudgetLimit {
    pub units: u32,
}

impl ActionTrait for SetComputeBudgetLimit {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}

#[action]
pub struct SetComputeUnitPrice {
    pub micro_lamports: u64,
}

impl ActionTrait for SetComputeUnitPrice {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
