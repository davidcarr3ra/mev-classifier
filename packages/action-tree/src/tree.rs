use actions::Action;
use indextree::{Arena, Descendants, Node, NodeEdge, NodeId};

mod display;

pub type ActionNodeId = NodeId;
pub type ActionNode = Node<Action>;
pub type ActionNodeEdge = NodeEdge;
pub type ActionDescendants<'a> = Descendants<'a, Action>;

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

    pub fn get_mut(&mut self, node_id: ActionNodeId) -> Option<&mut ActionNode> {
        self.arena.get_mut(node_id)
    }

    pub fn insert_child(&mut self, parent: ActionNodeId, action: Action) -> ActionNodeId {
        let child_id = parent.append_value(action, &mut self.arena);
        child_id
    }

    pub fn remove_subtree(&mut self, node: ActionNodeId) {
        node.remove_subtree(&mut self.arena);
    }

    /// Creates a new parent node and moves the given node under it.
    pub fn insert_parent(&mut self, node_id: ActionNodeId, action: Action) -> ActionNodeId {
        let node = self.arena.get_mut(node_id).unwrap();
        let sibling_id = node.previous_sibling();
        let parent_id = node.parent().expect("Node has no parent");

        node_id.detach(&mut self.arena);

        let new_parent_id = if let Some(sibling_id) = sibling_id {
            // Add node at end of tree and link to previous sibling
            let new_parent = self.arena.new_node(action);
            sibling_id.insert_after(new_parent, &mut self.arena);
            new_parent
        } else {
            // Insert node as first child of parent
            parent_id.append_value(action, &mut self.arena)
        };

        new_parent_id.append(node_id, &mut self.arena);

        new_parent_id
    }

    /// Replaces the parent of a node with a new parent which is an existing node.
    /// The detached node is inserted as the last sibling of the new parent.
    pub fn replace_parent(&mut self, child_id: ActionNodeId, new_parent_id: ActionNodeId) {
        child_id.detach(&mut self.arena);
        new_parent_id.append(child_id, &mut self.arena);
    }

    /// Inserts a parent for children, assuming they are all siblings. Preserves order of children.F
    pub fn insert_parent_for_children(
        &mut self,
        old_parent_id: ActionNodeId,
        children: Vec<ActionNodeId>,
        action: Action,
    ) -> ActionNodeId {
        assert!(!children.is_empty(), "No children to insert parent for");

        // Create vec of children in tree order
        // First, insert new parent for first child
        let mut ordered_children = Vec::with_capacity(children.len());
        for child_id in old_parent_id.children(&self.arena) {
            if children.contains(&child_id) {
                ordered_children.push(child_id);
            }
        }

        assert!(
            ordered_children.len() == children.len(),
            "Not all children found in parent"
        );

        // Insert new parent for first child
        let new_parent_id = self.insert_parent(ordered_children[0], action);

        // Replace remaining children with new parent
        for child_id in ordered_children.iter().skip(1) {
            self.replace_parent(*child_id, new_parent_id);
        }

        new_parent_id
    }

    pub fn num_children(&self, parent: ActionNodeId) -> usize {
        parent.children(&self.arena).count()
    }

    pub fn descendants<'a>(&'a self, parent: ActionNodeId) -> ActionDescendants {
        parent.descendants(&self.arena)
    }

    pub fn children(&self, node: ActionNodeId) -> impl Iterator<Item = ActionNodeId> + '_ {
        node.children(&self.arena)
    }
}
