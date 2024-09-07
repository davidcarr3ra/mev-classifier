use macros::action;

use crate::ActionTrait;

#[action]
pub struct StarAtlasAction {}

impl ActionTrait for StarAtlasAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
