use actions::{Action, ActionNodeId, ActionTree, DexSwap};
use classifier_core::{SandwichAttackTag, TransactionTag};
use std::collections::HashMap;
use thiserror::Error;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Error)]
enum ClassifySandwichAttackError {
    #[error("Error classifying sandwich attack")]
    UnexpectedError(#[from] anyhow::Error),
}

// Limitation: For now we only look for sandwich attacks in the same block
pub fn classify_sandwich_attack(root: ActionNodeId, tree: &mut ActionTree) {
    // Group swaps by token pair to simplify pattern matching
    let mut token_pair_groups: HashMap<String, Vec<(ActionNodeId, Pubkey, &DexSwap)>> = HashMap::new();

    // Collect all DEX swaps and group them by token pair
    for txn_id in tree.descendants(root) {
        if let Action::ClassifiableTransaction(txn) = tree.get(txn_id).unwrap().get() {
            for child_id in tree.descendants(txn_id) {
                if let Action::DexSwap(swap) = tree.get(child_id).unwrap().get() {

									// Pre-extract the owner address
									let owner_address = txn
									.static_keys
									.get(0)
									.expect("Expected at least one static key")
									.clone();

									// Group the DexSwap along with the static key
									// (ensure you update the type of your grouped structure accordingly)
									let mut token_pair_vec =
										vec![swap.input_mint.to_string(), swap.output_mint.to_string()];
									token_pair_vec.sort_unstable();
									let token_pair = token_pair_vec.join("-");
									token_pair_groups
										.entry(token_pair)
										.or_default()
										.push((txn_id, owner_address, swap));
                }
            }
        }
    }

    // Identify sandwich patterns within each token pair group
    let mut insertions: Vec<(ActionNodeId, TransactionTag)> = Vec::new();
    for token_pair_vec in token_pair_groups.values() {
        // Need at least 3 transactions for a sandwich pattern
        if token_pair_vec.len() < 3 {
            continue;
        }

        for i in 0..token_pair_vec.len() - 2 {
            let (front_id, front_address, front_tx) = token_pair_vec[i];
            let (victim_id, victim_address, victim_tx) = token_pair_vec[i + 1];
            let (back_id, back_address, back_tx) = token_pair_vec[i + 2];

            // Verify the sandwich pattern:
            if front_address == back_address // Same attacker
                && front_address != victim_address // Attacker != Victim
                && front_tx.input_mint == victim_tx.input_mint
                && front_tx.output_mint == victim_tx.output_mint
                && back_tx.input_mint == front_tx.output_mint
                && back_tx.output_mint == front_tx.input_mint
            {
                // Calculate profit in terms of the input token
                let profit = (back_tx.output_amount as i128 - front_tx.input_amount as i128) as i64;

                if profit > 0 {
                    // Append tags
                    insertions.push((
                        front_id,
                        TransactionTag::SandwichAttack(SandwichAttackTag::Frontrun {
                            token_bought: front_tx.output_mint,
                            amount: front_tx.output_amount,
                            attacker_pubkey: front_address,
                        }),
                    ));
                    insertions.push((
                        victim_id,
                        TransactionTag::SandwichAttack(SandwichAttackTag::Victim {
                            token_bought: victim_tx.output_mint,
                            amount: victim_tx.output_amount,
                            victim_pubkey: victim_address,
                        }),
                    ));
                    insertions.push((
                        back_id,
                        TransactionTag::SandwichAttack(SandwichAttackTag::Backrun {
                            token_sold: back_tx.input_mint,
                            amount: back_tx.input_amount,
                            attacker_pubkey: back_address,
                            profit_amount: profit,
                        }),
                    ));
                }
            }
        }
    }

    // println!("INSERTIONS: {:?}", insertions);

    for (child_id, tag) in insertions {
        if let Some(node) = tree.get_mut(child_id) {
            if let Action::ClassifiableTransaction(txn) = node.get_mut() {
                txn.tags.push(tag);
            }
            // Skip if not a ClassifiableTransaction - this can happen if classification failed
        }
    }
}
