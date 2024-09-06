use action_tree::{ActionNodeId, ActionTree};
use actions::{Action, JitoBundle};
use solana_sdk::signature::Signature;
use std::str::FromStr;

use crate::jito_bundle_client::fetch_jito_bundles;

pub struct PostProcessConfig {
    pub retain_votes: bool,
    pub remove_empty_transactions: bool,
    pub cluster_jito_bundles: bool,
}

pub fn post_process(config: PostProcessConfig, tree: &mut ActionTree) {
    // TODO: This currently assumes root is block node. This may not always be the case
    // if we are buffering past blocks for multi-block MEV inspection
    let root = tree.root();

    let mut remove_list = Vec::with_capacity(tree.num_children(root));
    let mut transaction_list = Vec::with_capacity(tree.num_children(root));

    // TODO: Use manual recursion instead of Descendants iterator
    // to avoid redundant remove list entries
    for node_id in tree.descendants(root) {
        let node = tree.get(node_id).unwrap().get();

        // Save transaction nodes for pruning
        match node {
            Action::ClassifiableTransaction(_) => {
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

    // TODO: This is removed because we have not configued the jito
    // bundle scraper for historical data yet. This will likely require some
    // pre-indexing of their bundle history as a separate process.
    // if config.cluster_jito_bundles {
    //     if let Err(e) = process_jito_bundles(root, tree) {
    //         eprintln!("Error processing Jito bundles: {}", e);
    //     }
    // }
}

#[allow(dead_code)]
fn process_jito_bundles(
    block_id: ActionNodeId,
    tree: &mut ActionTree,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundles = fetch_jito_bundles()?;

    for bundle in bundles {
        let bundle_node = JitoBundle {
            bundle_id: bundle.bundle_id,
            timestamp: bundle.timestamp,
            tippers: bundle.tippers,
            landed_tip_lamports: bundle.landed_tip_lamports,
        };

        let tx_ids = bundle
            .transactions
            .iter()
            .filter_map(|tx_hash| find_transaction_node(tree, tx_hash))
            .collect::<Vec<_>>();

        if !tx_ids.is_empty() {
            tree.insert_parent_for_children(block_id, tx_ids, bundle_node.into());
        }
    }

    Ok(())
}

fn search(tree: &ActionTree, node: ActionNodeId, tx_hash: &str) -> Option<ActionNodeId> {
    if let Some(action) = tree.get(node).map(|n| n.get()) {
        if let Action::ClassifiableTransaction(tx) = action {
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
