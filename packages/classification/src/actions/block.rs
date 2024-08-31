use macros::action;

use super::ActionTrait;

#[action]
pub struct Block {
    pub slot: u64,
}

impl Block {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

impl ActionTrait for Block {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("Instructions can not be blocks")
    }
}
