use crate::ActionTrait;
use macros::action_enum;
// use classifiers::misc_classifiers::phoenix_v1::Side;

#[action_enum]
pub enum PhoenixV1Action {
	Swap
}

impl ActionTrait for PhoenixV1Action {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}
