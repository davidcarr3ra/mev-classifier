use macros::action;

use super::ActionTrait;

#[action]
pub struct Block {
    pub slot: u64,
    pub parent_slot: u64,
    pub block_time: i64,

    pub total_base_fees: Option<u64>,
    pub total_priority_fees: Option<u64>,
    pub total_tips: Option<u64>,
}

impl Block {
    pub fn new(slot: u64, parent_slot: u64, block_time: i64) -> Self {
        Self {
            slot,
            parent_slot,
            block_time,
            total_base_fees: None,
            total_priority_fees: None,
            total_tips: None,
        }
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
