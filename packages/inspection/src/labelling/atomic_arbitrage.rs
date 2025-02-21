use actions::{Action, ActionNodeId, ActionTree};
use classifier_core::{AtomicArbitrageTag, TransactionTag};
use thiserror::Error;
use solana_sdk::pubkey::Pubkey;

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

        insertions.push((child_id, TransactionTag::AtomicArbitrage(atomic_arbitrage)));
    }

    for (child_id, tag) in insertions {
        match tree.get_mut(child_id).unwrap().get_mut() {
            Action::ClassifiableTransaction(txn) => {
                txn.tags.push(tag);
            }
            _ => unreachable!(),
        }
    }
}

/// Assess a transaction to determine if it is an atomic arbitrage.
/// This has a limitation that it can't find arbs in transactions with multiple arbs (very unlikely)
/// or an arbitrage across multiple transactions (have yet to disover this)
fn try_find_atomic_arb(
    tree: &ActionTree,
    txn_id: ActionNodeId,
) -> Result<Option<AtomicArbitrageTag>, ClassifyAtomicArbitrageError> {
    let mut first_swap = None;
    let mut last_swap = None;

    let swapper_address: Pubkey = match tree.get(txn_id).unwrap().get() {
        Action::ClassifiableTransaction(txn) => txn.static_keys[0],
        _ => unreachable!(),
    };

    for child in tree.descendants(txn_id) {
        let action_node = tree.get(child).unwrap();

        let action = action_node.get();

        match action {
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
        let profit_amount = last_swap.output_amount as i128 - first_swap.input_amount as i128;

        return Ok(Some(AtomicArbitrageTag {
            mint: first_swap.input_mint,
            profit_amount,
            address: swapper_address,
        }));
    }

    Ok(None)
}
