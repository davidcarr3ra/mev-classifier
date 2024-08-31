use indextree::{Arena, Descendants, Node, NodeEdge, NodeId};

use crate::actions::Action;

mod display;

pub type ActionNodeId = NodeId;
pub type ActionNode = Node<Action>;
pub type ActionNodeEdge = NodeEdge;

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

    pub fn get(&self, node_id: ActionNodeId) -> Option<&ActionNode> {
        self.arena.get(node_id)
    }

    pub fn insert(&mut self, parent: ActionNodeId, action: Action) -> ActionNodeId {
        let child_id = parent.append_value(action, &mut self.arena);
        child_id
    }

    pub fn remove_subtree(&mut self, node: ActionNodeId) {
        node.remove_subtree(&mut self.arena);
    }

    pub fn num_children(&self, parent: ActionNodeId) -> usize {
        parent.children(&self.arena).count()
    }

    pub fn descendants<'a>(&'a self, parent: ActionNodeId) -> Descendants<'a, Action> {
        parent.descendants(&self.arena)
    }

    pub fn children(&self, node: ActionNodeId) -> impl Iterator<Item = ActionNodeId> + '_ {
        node.children(&self.arena)
    }

    pub fn move_node(&mut self, node: ActionNodeId, new_parent: ActionNodeId) {
        node.remove(&mut self.arena);
        node.insert_after(new_parent, &mut self.arena);

    }
}
