use std::fmt::{Debug, Display};

use super::ActionTree;

impl<Action> Display for ActionTree<Action>
where
    Action: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = self.root();
        let pretty_print = root.debug_pretty_print(&self.arena);

        pretty_print.fmt(f)
    }
}
