use actions::{Action, JitoBundle};
use classifier_core::{ActionNodeId, ActionTree};
use solana_sdk::signature::Signature;
use std::str::FromStr;

use crate::jito_bundle_client::fetch_jito_bundles;

pub struct PostProcessConfig {
    pub retain_votes: bool,
    pub remove_empty_transactions: bool,
    pub cluster_jito_bundles: bool,
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

    if config.cluster_jito_bundles {
        if let Err(e) = process_jito_bundles(tree) {
            eprintln!("Error processing Jito bundles: {}", e);
        }
    }
}

fn process_jito_bundles(tree: &mut ActionTree) -> Result<(), Box<dyn std::error::Error>> {
    let bundles = fetch_jito_bundles()?;

    for bundle in bundles {
        let bundle_node = tree.insert(
            tree.root(),
            Action::JitoBundle(JitoBundle {
                bundle_id: bundle.bundle_id,
                timestamp: bundle.timestamp,
                tippers: bundle.tippers,
                landed_tip_lamports: bundle.landed_tip_lamports,
            }),
        );

        for tx_hash in bundle.transactions.iter() {
            if let Some(tx_node) = find_transaction_node(tree, tx_hash) {
                tree.move_node(tx_node, bundle_node);
            }
        }
    }

    Ok(())
}

fn search(tree: &ActionTree, node: ActionNodeId, tx_hash: &str) -> Option<ActionNodeId> {
    if let Some(action) = tree.get(node).map(|n| n.get()) {
        if let Action::Transaction(tx) = action {
            if tx.signature == Signature::from_str(tx_hash).ok()? {
                return Some(node);
            }
        }
    }

    for child in tree.children(node) {
        if let Some(found) = search(tree, child, tx_hash) {
            return Some(found);
        }
    }

    None
}

fn find_transaction_node(tree: &ActionTree, tx_hash: &str) -> Option<ActionNodeId> {
    search(tree, tree.root(), tx_hash)
}

// fn find_transaction_node(tree: &ActionTree, tx_hash: &str) -> Option<NodeId> {
//     let signature = Signature::from_str(tx_hash).ok()?;

//     tree.iter().find_map(|(node_id, action)| {
//         if let Action::Transaction(tx) = action {
//             if tx.signature == signature {
//                 Some(node_id)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     })
// }
