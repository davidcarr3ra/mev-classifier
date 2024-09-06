use action_tree::ActionTree;
use actions::{Action, Block};
use classifier_core::ClassifiableTransaction;
use solana_transaction_status::UiConfirmedBlock;
use thiserror::Error;

use crate::classify_transaction;

#[derive(Debug, Error)]
pub enum ClassifyBlockError {
    #[error("No transactions in block data")]
    MissingTransactions,

    #[error("No block time in block data")]
    MissingBlockTime,

    #[error("No meta data in transaction")]
    MissingTransactionMeta,

    #[error("Failed to classify transaction")]
    ClassifyError(#[from] crate::ClassifyError),
}

type Result<T> = std::result::Result<T, ClassifyBlockError>;

pub fn classify_block(slot: u64, block: UiConfirmedBlock) -> Result<ActionTree> {
    if block.transactions.is_none() {
        return Err(ClassifyBlockError::MissingTransactions);
    }

    let block_time = match block.block_time {
        Some(time) => time,
        None => {
            return Err(ClassifyBlockError::MissingBlockTime);
        }
    };

    let root_action = Block::new(slot, block.parent_slot, block_time);

    let mut tree = ActionTree::new(root_action.into());
    let block_id = tree.root();

    for txn in block.transactions.unwrap() {
        let v_txn = txn.transaction.decode().unwrap();

        if txn.meta.is_none() {
            return Err(ClassifyBlockError::MissingTransactionMeta);
        }

        let signature = v_txn.signatures.first().unwrap().clone();
        let c_txn = ClassifiableTransaction::new(v_txn, txn.meta.unwrap());
        let tx_id = tree.insert_child(block_id, c_txn.clone().into());

        match classify_transaction(c_txn, &mut tree, tx_id) {
            Ok(_) => {}
            Err(err) => {
                // TODO: Handle these somehow
                tracing::error!(
                    "Failed to classify transaction: {:?}, signature: {}",
                    err,
                    signature,
                );
            }
        }
    }

    calculate_rewards(&mut tree);

    Ok(tree)
}

pub const BASE_FEE: u64 = 5000;

fn calculate_rewards(tree: &mut ActionTree) {
    let block_id = tree.root();

    let mut total_tips = 0;
    let mut total_priority_fees = 0;
    let mut total_base_fees = 0;

    for node_id in tree.descendants(block_id) {
        let node = tree.get(node_id).unwrap();

        match node.get() {
            Action::ClassifiableTransaction(txn) => {
                if txn.fee > BASE_FEE {
                    total_base_fees += BASE_FEE;
                    total_priority_fees += txn.fee - BASE_FEE;
                } else {
                    total_base_fees += txn.fee;
                }
            }
            Action::JitoTip(tip) => {
                // Find parent transaction
                let mut succeeded = false;
                let mut parent_id = node.parent();
                while let Some(some_parent_id) = parent_id {
                    let parent = tree.get(some_parent_id).unwrap();

                    match parent.get() {
                        Action::ClassifiableTransaction(tx) => {
                            succeeded = tx.status.is_ok();
                            break;
                        }
                        _ => {
                            parent_id = parent.parent();
                        }
                    }
                }

                if succeeded {
                    total_tips += tip.tip_amount;
                }
            }
            _ => continue,
        };
    }

    let block = tree.get_mut(block_id).unwrap();
    let block = match block.get_mut() {
        Action::Block(block) => block,
        _ => unreachable!("Root node should be a block"),
    };

    block.total_base_fees = Some(total_base_fees);
    block.total_priority_fees = Some(total_priority_fees);
    block.total_tips = Some(total_tips);
}
