use super::Action;

#[derive(Debug, PartialEq, Eq)]
pub struct JitoBundleAction {
    pub id: String,
    pub timestamp: String,
    pub tippers: Vec<String>,
    pub landed_tip_lamports: u64,
}

impl JitoBundleAction {
    pub fn new(id: String, timestamp: String, tippers: Vec<String>, landed_tip_lamports: u64) -> Self {
        Self {
            id,
            timestamp,
            tippers,
            landed_tip_lamports,
        }
    }
}

impl Into<Action> for JitoBundleAction {
    fn into(self) -> Action {
        Action::JitoBundle(self)
    }
}