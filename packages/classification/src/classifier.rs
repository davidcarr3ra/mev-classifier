use crate::{
    protocols::classify_instruction,
    transaction::ClassifiableTransaction,
    tree::{ActionNodeId, ActionTree},
};

pub fn classify_transaction(
    txn: ClassifiableTransaction,
    tree: &mut ActionTree,
    parent: ActionNodeId,
) {
    for idx in 0..txn.instructions.len() {
        let classify_result = classify_instruction(&txn, idx, tree, parent);

        match classify_result {
            Ok(_) => {}
            Err(err) => {
                tracing::trace!("Failed to classify instruction: {:?}", err);
            }
        }
    }
}
