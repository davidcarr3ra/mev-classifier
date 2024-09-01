use actions::Action;
use classifier_core::ActionTree;

pub struct PostProcessConfig {
    pub retain_votes: bool,
    pub remove_empty_transactions: bool,
}

pub fn post_process(config: PostProcessConfig, tree: &mut ActionTree) {
    let root = tree.root();
    let mut remove_list = Vec::with_capacity(tree.num_children(root));
    let mut transaction_list = Vec::with_capacity(tree.num_children(root));

    // TODO: Use manual recursion instead of Descendants iterator
    // to avoid redundant remove list entries
    for node_id in tree.descendants(root) {
        let node = tree.get(node_id).unwrap().get();

        // Save transaction nodes for pruning
        match node {
            Action::Transaction(_) => {
                transaction_list.push(node_id);
            }
            _ => {}
        }

        // Flag nodes for removal
        let remove = match node {
            Action::Vote(_) => !config.retain_votes,
            _ => false,
        };

        if remove {
            remove_list.push(node_id);
        }
    }

    for remove in remove_list {
        // TODO: See above to make this faster and avoid
        // a memory fetch for each node
        if tree.get(remove).is_none() {
            continue;
        }

        tree.remove_subtree(remove);
    }

    // Remove transactions if no children remain
    if config.remove_empty_transactions {
        for transaction in transaction_list {
            if tree.num_children(transaction) == 0 {
                tree.remove_subtree(transaction);
            }
        }
    }
}
