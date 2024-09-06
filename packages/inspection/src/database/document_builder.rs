use action_tree::{ActionDescendants, ActionNodeId, ActionTree};
use actions::{Action, ActionTrait};
use classifier_core::ClassifiableTransaction;
use mongodb::bson::{self};
use thiserror::Error;

use super::mongo_actions::MongoSerialize;

#[derive(Debug, Error)]
pub enum DocumentBuilderError {
    #[error("Invalid block ID")]
    InvalidBlockId,

    #[error("Invalid root node")]
    InvalidRootNode,

    #[error("No transactions found")]
    NoTransactions,
}

type Result<T> = std::result::Result<T, DocumentBuilderError>;

pub struct BlockDocuments {
    pub block: bson::Document,
    pub transactions: Vec<bson::Document>,
    pub block_metadata: Vec<bson::Document>,
}

pub fn build_block_documents(tree: &ActionTree, block_id: ActionNodeId) -> Result<BlockDocuments> {
    // Build the documents for the block
    let block_node = tree
        .get(block_id)
        .ok_or_else(|| DocumentBuilderError::InvalidBlockId)?;

    let block_action = block_node.get();
    let block_action = match block_action {
        Action::Block(block) => block,
        _ => return Err(DocumentBuilderError::InvalidBlockId),
    };

    // Build documents for all child nodes of block.
    let mut transaction_documents = Vec::new();
    let mut block_metadata = Vec::new();
    let mut ordering = 0;
    for child_id in tree.children(block_id) {
        let (transactions, block_metadatas) =
            build_action_stack(block_action.slot as u32, &mut ordering, tree, child_id)?;
        transaction_documents.extend(transactions);
        block_metadata.extend(block_metadatas);
    }

    let block_document = block_action
        .metadata_bson()
        .ok_or_else(|| DocumentBuilderError::InvalidRootNode)?;

    Ok(BlockDocuments {
        block: block_document,
        transactions: transaction_documents,
        block_metadata,
    })
}

fn build_action_stack(
    slot_height: u32,
    ordering: &mut usize,
    tree: &ActionTree,
    root_id: ActionNodeId,
) -> Result<(Vec<bson::Document>, Vec<bson::Document>)> {
    let root = tree.get(root_id).unwrap().get();
    if !root.is_document_root() {
        return Err(DocumentBuilderError::InvalidRootNode);
    }

    let mut descendants_iter = tree.descendants(root_id).peekable();
    let mut root_metadata = Vec::new();
    let mut transactions = Vec::new();

    while let Some(descendant_id) = descendants_iter.next() {
        let descendant_node = tree.get(descendant_id).unwrap();
        let descendant = descendant_node.get();

        match descendant {
            // Store transaction metadata in transaction metadata vec
            Action::ClassifiableTransaction(tx) => {
                let tx_meta = build_transaction_document(
                    slot_height,
                    *ordering,
                    tx,
                    &mut descendants_iter,
                    tree,
                    descendant_id,
                );
                transactions.push(tx_meta);

                *ordering += 1;
            }
            // Node is parent to transaction node, store its root metadata
            _ => {
                let mut metadata = match descendant.metadata_bson() {
                    Some(metadata) => metadata,
                    None => continue,
                };
                
                metadata.insert("_id", bson::oid::ObjectId::new());
                metadata.insert("block_id", slot_height);
                root_metadata.push(metadata);
            }
        };
    }

    if transactions.is_empty() {
        return Err(DocumentBuilderError::NoTransactions);
    }

    Ok((transactions, root_metadata))
}

/// Constructs inner document for one transaction's call stack actions.
/// This does not include outer metadata such as AtomicArbitrage or JitoBundle.
fn build_transaction_document(
    slot_height: u32,
    ordering: usize,
    tx: &ClassifiableTransaction,
    descendants_iter: &mut std::iter::Peekable<ActionDescendants>,
    tree: &ActionTree,
    tx_id: ActionNodeId,
) -> bson::Document {
    let mut parent_stack = vec![tx_id];
    let mut transaction_metadata = vec![];

    while let Some(descendant_id) = descendants_iter.peek() {
        let descendant_node = tree.get(*descendant_id).unwrap();

        let parent_id = descendant_node.parent().unwrap();
        while parent_stack.last() != Some(&parent_id) {
            parent_stack.pop();
        }

        if parent_stack.is_empty() {
            break;
        }

        // Advance iterator now that this node is confirmed to be in
        // transaction subtree
        parent_stack.push(*descendant_id);
        descendants_iter.next();

        // Attempt to store metadata
        let descendant = descendant_node.get();
        let mut metadata = match descendant.metadata_bson() {
            Some(metadata) => metadata,
            None => continue,
        };

        metadata.insert("tx_stack_height", parent_stack.len() as u32 - 1);
        transaction_metadata.push(metadata);
    }

    let mut document = tx.metadata_bson().unwrap();
    document.insert("_id", bson::oid::ObjectId::new());
    document.insert("metadata", transaction_metadata);
    document.insert("block_order", ordering as u32);
    document.insert("block_id", slot_height);

    document
}
