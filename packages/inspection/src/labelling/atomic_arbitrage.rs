use action_tree::{ActionNodeId, ActionTree};
use actions::{Action, AtomicArbitrage};
use classifier_core::ClassifiableTransaction;
use thiserror::Error;

#[derive(Debug, Error)]
enum ClassifyAtomicArbitrageError {
    #[error("Missing pre-token balance")]
    MissingPreTokenBalance,

    #[error("Missing post-token balance")]
    MissingPostTokenBalance,

    #[error("Failed to parse amount")]
    ParseAmount(#[from] std::num::ParseIntError),
}

pub fn classify_atomic_arbitrage(root: ActionNodeId, tree: &mut ActionTree) {
    let mut insertions = Vec::new();

    for child_id in tree.descendants(root) {
        let action = tree.get(child_id).unwrap().get();

        let transaction = match action {
            Action::ClassifiableTransaction(txn) => txn,
            _ => continue,
        };

        let atomic_arbitrage = match try_find_atomic_arb(tree, child_id, transaction) {
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
    txn: &ClassifiableTransaction,
) -> Result<Option<AtomicArbitrage>, ClassifyAtomicArbitrageError> {
    let mut first_swap = None;
    let mut last_swap = None;

    for child in tree.descendants(txn_id) {
        let action = tree.get(child).unwrap().get();

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
        let pre_balance = txn
            .get_pre_token_balance(&first_swap.input_token_account)
            .map_err(|_| ClassifyAtomicArbitrageError::MissingPreTokenBalance)?;

        let post_balance = txn
            .get_post_token_balance(&first_swap.output_token_account)
            .map_err(|_| ClassifyAtomicArbitrageError::MissingPostTokenBalance)?;

        let in_amount = u64::from_str_radix(&pre_balance.ui_token_amount.amount, 10)
            .map_err(|e| ClassifyAtomicArbitrageError::ParseAmount(e))?;

        let out_amount = u64::from_str_radix(&post_balance.ui_token_amount.amount, 10)
            .map_err(|e| ClassifyAtomicArbitrageError::ParseAmount(e))?;

        let profit_amount = out_amount - in_amount;

        return Ok(Some(AtomicArbitrage::new(
            first_swap.input_mint,
            profit_amount,
        )));
    }

    Ok(None)
}
