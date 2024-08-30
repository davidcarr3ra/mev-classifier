use super::Action;

#[derive(Debug, PartialEq, Eq)]
pub struct BlockAction {
    pub slot: u64,
}

impl BlockAction {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

impl Into<Action> for BlockAction {
    fn into(self) -> Action {
        Action::Block(self)
    }
}
