use actions::{Action, ActionNodeId, ActionTree, AtomicArbitrage};
use thiserror::Error;

#[derive(Debug, Error)]
enum ClassifyAtomicArbitrageError {
    #[error("Failed to parse amount")]
    ParseAmount(#[from] std::num::ParseIntError),
}

pub fn classify_atomic_arbitrage(root: ActionNodeId, tree: &mut ActionTree) {
    let mut insertions = Vec::new();

    for child_id in tree.descendants(root) {
        let action = tree.get(child_id).unwrap().get();

        match action {
            Action::ClassifiableTransaction(_) => {}
            _ => continue,
        };

        let atomic_arbitrage = match try_find_atomic_arb(tree, child_id) {
            Ok(Some(atomic_arbitrage)) => atomic_arbitrage,
            Err(e) => {
                tracing::error!("Failed to classify atomic arbitrage: {:?}", e);
                continue;
            }
            _ => continue,
        };

        insertions.push((child_id, atomic_arbitrage));
    }

    for (txn_id, atomic_arbitrage) in insertions {
        tree.insert_parent(txn_id, atomic_arbitrage.into());
    }
}

/// Assess a transaction to determine if it is an atomic arbitrage.
/// This has a limitation that it can't find arbs in transactions with multiple arbs (very unlikely)
/// or an arbitrage across multiple transactions (have yet to disover this)
fn try_find_atomic_arb(
    tree: &ActionTree,
    txn_id: ActionNodeId,
) -> Result<Option<AtomicArbitrage>, ClassifyAtomicArbitrageError> {
    let mut first_swap = None;
    let mut last_swap = None;

    // let mut parent_stack = vec![txn_id];

    for child in tree.descendants(txn_id) {
        let action_node = tree.get(child).unwrap();
        // let parent_id = action_node.parent().unwrap();
        // if &parent_id != parent_stack.last().unwrap() {
        //     parent_stack.push(parent_id);
        // }

        let action = action_node.get();

        match action {
            Action::ClassifiableTransaction(txn) => {
                if txn.status.is_err() {
                    return Ok(None);
                }
            }
            Action::DexSwap(swap) => {
                if first_swap.is_none() {
                    first_swap = Some(swap);
                }

                last_swap = Some(swap);
            }
            _ => {}
        }
    }

    let first_swap = match first_swap {
        Some(swap) => swap,
        None => return Ok(None),
    };

    let last_swap = last_swap.unwrap();

    if first_swap.input_mint == last_swap.output_mint {
        let profit_amount = if last_swap.output_amount > first_swap.input_amount {
            last_swap.output_amount - first_swap.input_amount
        } else {
            // TODO: Classify failed arbitrage?
            0
        };

        return Ok(Some(AtomicArbitrage::new(
            first_swap.input_mint,
            profit_amount,
        )));
    }

    Ok(None)
}
