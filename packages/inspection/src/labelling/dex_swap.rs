use action_tree::{ActionNodeId, ActionTree};
use actions::{Action, DexSwap};

pub fn classify_dex_swaps(root: ActionNodeId, tree: &mut ActionTree) {
    let mut parent_txn = None;

    let mut insertions = Vec::new();

    for child_id in tree.descendants(root) {
        let action = tree.get(child_id).unwrap().get();

        match action {
            Action::ClassifiableTransaction(txn) => {
                parent_txn = Some(txn);
                continue;
            }
            _ => {}
        }

        let parent_txn = match parent_txn {
            Some(txn) => txn,
            None => continue,
        };

        let dex_swap = match DexSwap::try_from(action, parent_txn) {
            Some(dex_swap) => dex_swap,
            None => continue,
        };

        insertions.push((child_id, dex_swap));
    }

    for (child_id, dex_swap) in insertions {
        tree.insert_parent(child_id, dex_swap.into());
    }
}
