use actions::{Action, ActionNodeId, ActionTree, DexSwap};
use std::collections::HashMap;
// use solana_sdk::pubkey::Pubkey;
use thiserror::Error;
use classifier_core::{TransactionTag, SandwichAttackTag};

#[derive(Debug, Error)]
enum ClassifySandwichAttackError {
    #[error("Error classifying sandwich attack")]
    UnexpectedError(#[from] anyhow::Error),
}

// #[derive(Debug)]
// pub struct Sandwich {
//     pub attacker_pubkey: Pubkey,
//     pub token_bought: Pubkey,
//     pub token_sold: Pubkey,
//     pub profit: i64,
// }

// // Note: For now we only look for sandwich attacks in the same block
// pub fn classify_sandwich(root: ActionNodeId, tree: &mut ActionTree) -> Vec<Sandwich> {
//     // Group swaps by token pair to simplify pattern matching
//     let mut token_pair_groups: HashMap<String, Vec<&DexSwap>> = HashMap::new();

//     // Collect all DEX swaps and group them by token pair
//     for child_id in tree.descendants(root) {
//         if let Action::DexSwap(swap) = tree.get(child_id).unwrap().get() {
//             let mut token_pair_vec = vec![swap.input_mint.to_string(), swap.output_mint.to_string()];
//             token_pair_vec.sort_unstable();
//             let token_pair = token_pair_vec.join("-");
//             token_pair_groups.entry(token_pair).or_default().push(swap);
//         }
//     }

//     // Identify sandwich patterns within each token pair group
//     let mut sandwiches = Vec::new();
//     for token_pair_vec in token_pair_groups.values() {
//         // Need at least 3 transactions for a sandwich pattern
//         if token_pair_vec.len() < 3 {
//             continue;
//         }
        
//         for i in 0..token_pair_vec.len()-2 {
//             let front_tx = token_pair_vec[i];
//             let victim_tx = token_pair_vec[i+1];
//             let back_tx = token_pair_vec[i+2];

//             // Verify the sandwich pattern:
//             // Front and victim buy token A with token B, back sells token A for token B
//             if front_tx.input_mint == victim_tx.input_mint 
//                 && front_tx.output_mint == victim_tx.output_mint
//                 && back_tx.input_mint == front_tx.output_mint
//                 && back_tx.output_mint == front_tx.input_mint
//             {
//                 // Calculate profit in terms of the input token
//                 let profit = (back_tx.output_amount as i128 - front_tx.input_amount as i128) as i64;

//                 sandwiches.push(Sandwich {
//                     attacker_pubkey: front_tx.input_token_account,
//                     token_bought: front_tx.output_mint,
//                     token_sold: front_tx.input_mint,
//                     profit,
//                 });
//             }
//         }
//     }

//     sandwiches
// }

// Note: For now we only look for sandwich attacks in the same block
// We also only look at DEX swaps that are descendants of ClassifiableTransactions
pub fn classify_sandwich_attack(root: ActionNodeId, tree: &mut ActionTree) {
    // Group swaps by token pair to simplify pattern matching
    let mut token_pair_groups: HashMap<String, Vec<(ActionNodeId, &DexSwap)>> = HashMap::new();

    // Collect all DEX swaps and group them by token pair
    for txn_id in tree.descendants(root) {
			if let Action::ClassifiableTransaction(_) = tree.get(txn_id).unwrap().get() {
				for child_id in tree.descendants(txn_id) {
					if let Action::DexSwap(swap) = tree.get(child_id).unwrap().get() {
						let mut token_pair_vec = vec![swap.input_mint.to_string(), swap.output_mint.to_string()];
						token_pair_vec.sort_unstable();
						let token_pair = token_pair_vec.join("-");
						token_pair_groups.entry(token_pair).or_default().push((txn_id, swap));
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

        for i in 0..token_pair_vec.len()-2 {
            let (front_id, front_tx) = token_pair_vec[i];
            let (victim_id, victim_tx) = token_pair_vec[i+1];
            let (back_id, back_tx) = token_pair_vec[i+2];

            // Verify the sandwich pattern:
            // Front and victim buy token A with token B, back sells token A for token B
            if front_tx.input_mint == victim_tx.input_mint
                && front_tx.output_mint == victim_tx.output_mint
                && back_tx.input_mint == front_tx.output_mint
                && back_tx.output_mint == front_tx.input_mint
            {
                // Calculate profit in terms of the input token
                let profit = (back_tx.output_amount as i128 - front_tx.input_amount as i128) as i64;

                // Append tags
                insertions.push((
                    front_id,
                    TransactionTag::SandwichAttack(SandwichAttackTag::Frontrun {
                        token_bought: front_tx.output_mint,
                        amount: front_tx.output_amount,
                        attacker_pubkey: front_tx.input_token_account,
                    }),
                ));
                insertions.push((
                    victim_id,
                    TransactionTag::SandwichAttack(SandwichAttackTag::Victim {
                        token_bought: victim_tx.output_mint,
                        amount: victim_tx.output_amount,
                        victim_pubkey: victim_tx.input_token_account,
                    }),
                ));
                insertions.push((
                    back_id,
                    TransactionTag::SandwichAttack(SandwichAttackTag::Backrun {
                        token_sold: back_tx.input_mint,
                        amount: back_tx.input_amount,
                        attacker_pubkey: back_tx.output_token_account,
                        profit_amount: profit,
                    }),
                ));
            }
        }
    }

    for (child_id, tag) in insertions {
        if let Some(node) = tree.get_mut(child_id) {
            if let Action::ClassifiableTransaction(txn) = node.get_mut() {
                txn.tags.push(tag);
            }
            // Skip if not a ClassifiableTransaction - this can happen if classification failed
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actions::Block;
//     use std::time::{SystemTime, UNIX_EPOCH};
// 		use solana_sdk::pubkey::Pubkey;

// 		#[test]
//     fn test_classify_sandwich_mock_block() {
//         // Create a mock block
//         let current_time = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_secs() as i64;
//         let block = Block::new(100, 99, current_time);
//         let mut tree = ActionTree::new(Action::Block(block));
//         let root = tree.root();
        
//         // Set up test accounts and tokens
//         let token_a = Pubkey::new_unique();
//         let token_b = Pubkey::new_unique();
//         let attacker = Pubkey::new_unique();
//         let victim = Pubkey::new_unique();
//         let dex = Pubkey::new_unique();
        
//         // Create a sandwich attack pattern
//         let swaps = [
//             // Front-run: Attacker buys token B with token A
//             DexSwap {
//                 input_mint: token_a,
//                 output_mint: token_b,
//                 input_amount: 1000,
//                 output_amount: 100,
//                 input_token_account: attacker,
//                 output_token_account: dex,
//             },
//             // Victim swap: Same direction as front-run
//             DexSwap {
//                 input_mint: token_a,
//                 output_mint: token_b,
//                 input_amount: 9000,
//                 output_amount: 900,
//                 input_token_account: victim,
//                 output_token_account: dex,
//             },
//             // Back-run: Attacker sells token B for token A
//             DexSwap {
//                 input_mint: token_b,
//                 output_mint: token_a,
//                 input_amount: 100,
//                 output_amount: 1200,
//                 input_token_account: attacker,
//                 output_token_account: dex,
//             },
//         ];
        
//         // Add swaps to tree
//         for swap in swaps {
//             tree.insert_child(root, Action::DexSwap(swap));
//         }

//         // Test sandwich detection
//         classify_sandwich_attack(root, &mut tree);

//         assert_eq!(sandwiches.len(), 1, "Expected to find exactly one sandwich attack");
//         let sandwich = &sandwiches[0];
//         assert_eq!(sandwich.attacker_pubkey, attacker);
//         assert_eq!(sandwich.token_bought, token_b);
//         assert_eq!(sandwich.token_sold, token_a);
//         assert_eq!(sandwich.profit, 200);
//     }

//     // #[test]
//     // fn test_classify_sandwich_mock_block_old() {
//     //     // Create a mock block
//     //     let current_time = SystemTime::now()
//     //         .duration_since(UNIX_EPOCH)
//     //         .unwrap()
//     //         .as_secs() as i64;
//     //     let block = Block::new(100, 99, current_time);
//     //     let mut tree = ActionTree::new(Action::Block(block));
//     //     let root = tree.root();
        
//     //     // Set up test accounts and tokens
//     //     let token_a = Pubkey::new_unique();
//     //     let token_b = Pubkey::new_unique();
//     //     let attacker = Pubkey::new_unique();
//     //     let victim = Pubkey::new_unique();
//     //     let dex = Pubkey::new_unique();
        
//     //     // Create a sandwich attack pattern
//     //     let swaps = [
//     //         // Front-run: Attacker buys token B with token A
//     //         DexSwap {
//     //             input_mint: token_a,
//     //             output_mint: token_b,
//     //             input_amount: 1000,
//     //             output_amount: 100,
//     //             input_token_account: attacker,
//     //             output_token_account: dex,
//     //         },
//     //         // Victim swap: Same direction as front-run
//     //         DexSwap {
//     //             input_mint: token_a,
//     //             output_mint: token_b,
//     //             input_amount: 9000,
//     //             output_amount: 900,
//     //             input_token_account: victim,
//     //             output_token_account: dex,
//     //         },
//     //         // Back-run: Attacker sells token B for token A
//     //         DexSwap {
//     //             input_mint: token_b,
//     //             output_mint: token_a,
//     //             input_amount: 100,
//     //             output_amount: 1200,
//     //             input_token_account: attacker,
//     //             output_token_account: dex,
//     //         },
//     //     ];
        
//     //     // Add swaps to tree
//     //     for swap in swaps {
//     //         tree.insert_child(root, Action::DexSwap(swap));
//     //     }

//     //     // Test sandwich detection
//     //     let sandwiches = classify_sandwich(root, &mut tree);

//     //     assert_eq!(sandwiches.len(), 1, "Expected to find exactly one sandwich attack");
//     //     let sandwich = &sandwiches[0];
//     //     assert_eq!(sandwich.attacker_pubkey, attacker);
//     //     assert_eq!(sandwich.token_bought, token_b);
//     //     assert_eq!(sandwich.token_sold, token_a);
//     //     assert_eq!(sandwich.profit, 200);
//     // }

//     // #[test]
//     // fn test_classify_sandwich_real_block() {
//     //     use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
//     //     use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
//     //     use classifier_handler::classify_block;

//     //     // Initialize RPC client
//     //     let rpc_url = "https://api.mainnet-beta.solana.com";
//     //     let client = RpcClient::new(rpc_url);

//     //     // Choose a slot to analyze (you can replace this with any slot number)
//     //     let slot = 239939769;

//     //     // Fetch block
//     //     let block_config = RpcBlockConfig {
//     //         encoding: Some(UiTransactionEncoding::Base64),
//     //         transaction_details: Some(TransactionDetails::Full),
//     //         rewards: Some(true),
//     //         max_supported_transaction_version: Some(0),
//     //         commitment: None,
//     //     };

//     //     println!("Fetching block {}...", slot);
//     //     let block = client.get_block_with_config(slot, block_config).unwrap();
        
//     //     // Create and classify action tree
//     //     println!("Classifying block...");
// 		// 		let mut tree = match classify_block(slot, block, None) {
// 		// 				Ok(tree) => tree,
// 		// 				Err(err) => {
// 		// 						eprintln!("Failed to classify block: {:?}", err);
// 		// 						return;
// 		// 				}
// 		// 		};

// 		// 		// Label tree
//     //     crate::label_tree(&mut tree);

// 		// 		println!("Classifying sandwiches...");
//     //     let _sandwiches = classify_sandwich(tree.root(), &mut tree);
//     // }
// }