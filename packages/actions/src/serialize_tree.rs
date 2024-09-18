use action_tree::ActionNodeId;

use crate::{ActionTrait, ActionTree};

/// Currently assumes root node is a Block action. This is subject to change.
pub fn serialize_block(tree: &ActionTree, block_id: ActionNodeId) -> serde_json::Value {
    let mut descendants = tree.descendants(block_id);

    let block = descendants.next().unwrap();
    let value = tree.get(block).unwrap().get().to_json();

    // Stack to track node traversal
    let mut parent_stack = vec![block];
    let mut pruning = None;

    // Stack to track JSON serialization
    let mut json_stack = vec![(block, value)];

    let mut i = 0;

    for node_id in descendants {
        i += 1;

        let node = tree.get(node_id).unwrap();
        let action = node.get();

        // Track heirarchy of nodes
        let parent = node.parent().unwrap();

        update_stacks(
            &mut parent_stack,
            &mut json_stack,
            node_id,
            parent,
            &mut pruning,
        );

        // Skip pruned branches
        if pruning.is_some() {
            continue;
        }

        if action.serializable() {
            let mut node_json = action.to_json();
            node_json
                .as_object_mut()
                .unwrap()
                .insert("id".to_string(), serde_json::json!(i - 1));

            json_stack.push((node_id, node_json));
        } else {
            // Skip all children of non-serializable nodes
            pruning = Some(parent);
        }
    }

    while json_stack.len() > 1 {
        consolidate_json_values(&mut json_stack);
    }

    json_stack.pop().unwrap().1
}

fn update_stacks(
    parent_stack: &mut Vec<ActionNodeId>,
    json_stack: &mut Vec<(ActionNodeId, serde_json::Value)>,
    node_id: ActionNodeId,
    current_parent: ActionNodeId,
    pruning: &mut Option<ActionNodeId>,
) {
    while parent_stack.last().unwrap() != &current_parent {
        let popped = parent_stack.pop().unwrap();

        // If parent is no longer being iterated, stop pruning
        if let Some(some_pruning) = pruning {
            if popped == *some_pruning {
                *pruning = None;
            }
        }

        // Consolidate JSON values into master node
        if popped == json_stack.last().unwrap().0 {
            consolidate_json_values(json_stack);
        }
    }

    parent_stack.push(node_id);

    // If we have arrived at the next sibling of the parent prune from,
    // we can stop pruning
    if let Some(some_pruning) = pruning {
        if current_parent == *some_pruning {
            *pruning = None;
        }
    }
}

fn consolidate_json_values(json_stack: &mut Vec<(ActionNodeId, serde_json::Value)>) {
    let json_popped = json_stack.pop().unwrap();
    let parent_json = json_stack.last_mut().unwrap();

    // Append JSON value to parent "children" array
    parent_json
        .1
        .as_object_mut()
        .unwrap()
        .entry("children")
        .or_insert(serde_json::Value::Array(Vec::new()))
        .as_array_mut()
        .unwrap()
        .push(json_popped.1);
}
