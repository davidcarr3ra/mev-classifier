use indextree::{Arena, NodeId, Traverse};

use crate::actions::Action;

mod display;

pub type ActionNodeId = NodeId;

pub struct ActionTree {
    arena: Arena<Action>,
    root_id: ActionNodeId,
}

impl ActionTree {
    pub fn new(root: Action) -> Self {
        let mut arena = Arena::new();

        let root_id = arena.new_node(root);

        Self { arena, root_id }
    }

    pub fn root(&self) -> ActionNodeId {
        self.root_id
    }

    pub fn insert(&mut self, parent: ActionNodeId, action: Action) -> ActionNodeId {
        let child_id = parent.append_value(action, &mut self.arena);
        child_id
    }

    pub fn num_children(&self, parent: ActionNodeId) -> usize {
        parent.children(&self.arena).count()
    }

    pub fn traverse<'a>(&'a self, node: ActionNodeId) -> Traverse<'a, Action> {
        node.traverse(&self.arena)
    }
}
