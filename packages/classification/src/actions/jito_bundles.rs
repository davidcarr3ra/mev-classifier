use super::Action;

#[derive(Debug, PartialEq, Eq)]
pub struct JitoBundleAction {
    pub bundleId: String,
    pub timestamp: String,
    pub tippers: Vec<String>,
    pub landedTipLamports: u64,
}

impl JitoBundleAction {
    pub fn new(bundleId: String, timestamp: String, tippers: Vec<String>, landedTipLamports: u64) -> Self {
        Self {
            bundleId,
            timestamp,
            tippers,
            landedTipLamports,
        }
    }
}

impl Into<Action> for JitoBundleAction {
    fn into(self) -> Action {
        Action::JitoBundle(self)
    }
}