use macros::action;

use crate::ActionTrait;

#[action]
pub struct JitoBundle {
    pub bundle_id: String,
    pub timestamp: String,
    pub tippers: Vec<String>,
    pub landed_tip_lamports: u64,
}

impl JitoBundle {
    pub fn new(
        bundle_id: String,
        timestamp: String,
        tippers: Vec<String>,
        landed_tip_lamports: u64,
    ) -> Self {
        Self {
            bundle_id,
            timestamp,
            tippers,
            landed_tip_lamports,
        }
    }
}

impl ActionTrait for JitoBundle {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("Bundles should only be created in post-processing")
    }
}
