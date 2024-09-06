use macros::action;

use super::ActionTrait;

#[action]
pub struct Block {
    pub slot: u64,
    pub block_time: i64,
}

impl Block {
    pub fn new(slot: u64, block_time: i64) -> Self {
        Self { slot, block_time }
    }
}

impl ActionTrait for Block {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("Instructions can not be blocks")
    }

    fn is_document_root(&self) -> bool {
        unreachable!("Blocks should not be document roots")
    }
}
