use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use action_tree::ActionNodeId;

use crate::ActionTree;
use crate::serialize_block;

#[derive(Serialize, Debug)]
pub struct FlatChild {
    pub block_slot: Option<i64>,
    pub transaction_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub id: Option<i64>,
    pub input_amount: Option<i64>,
    pub input_mint: Option<String>,
    pub input_token_account: Option<String>,
    pub output_amount: Option<i64>,
    pub output_mint: Option<String>,
    pub output_token_account: Option<String>,
    pub program_id: Option<String>,
    pub tip_amount: Option<i64>,
    pub tipper: Option<String>,
    pub type_field: Option<String>,
    pub level: u8,
    pub path: String,
}


/// Recursively traverse the children array and flatten the structure.
///
/// * `parent_slot` - the parent's identifier (or any reference, e.g. slot)
/// * `children` - a slice of JSON values representing children nodes
/// * `level` - the current depth (starting at 0 for top-level children)
/// * `path` - a string representing the position in the hierarchy (e.g. "0", "0.1", etc.)
/// * `flat_records` - a mutable vector to which the flat records are appended
pub fn flatten_children(
    block_slot: Option<i64>,
    transaction_id: Option<i64>,
    parent_id: Option<i64>,
    children: &[Value],
    level: u8,
    path: String,
    flat_records: &mut Vec<FlatChild>,
) {
    for (idx, child) in children.iter().enumerate() {
        // Build a new path. For example, "0.1" indicates the second child of the first child.
        let current_path = if path.is_empty() {
            format!("{}", idx)
        } else {
            format!("{}.{}", path, idx)
        };

        // Extract the known fields from the JSON
        let flat = FlatChild {
            block_slot,
            transaction_id,
            parent_id,
            id: child.get("id").and_then(|v| v.as_i64()),
            input_amount: child.get("inputAmount").and_then(|v| v.as_i64()),
            input_mint: child.get("inputMint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            input_token_account: child
                .get("inputTokenAccount")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            output_amount: child.get("outputAmount").and_then(|v| v.as_i64()),
            output_mint: child.get("outputMint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            output_token_account: child
                .get("outputTokenAccount")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            program_id: child.get("programId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            tip_amount: child.get("tipAmount").and_then(|v| v.as_i64()),
            tipper: child.get("tipper").and_then(|v| v.as_str()).map(|s| s.to_string()),
            type_field: child.get("type").and_then(|v| v.as_str()).map(|s| s.to_string()),
            level,
            path: current_path.clone(),
        };

        flat_records.push(flat);

        // If the current child has its own children array, recursively flatten it.
        if let Some(nested_children) = child.get("children").and_then(|v| v.as_array()) {
            // For the nested level, you might choose to use the current child's `id` as the new parent reference.
            let new_parent = child.get("id").and_then(|v| v.as_i64()).or(parent_id);
            flatten_children(block_slot, transaction_id, new_parent, nested_children, level + 1, current_path, flat_records);
        }
    }
}


pub fn serialize_block_flat(tree: &ActionTree, block_id: ActionNodeId) ->  serde_json::Value {
    // First, serialize the block to the nested JSON using your existing logic.
    let nested_json: Value = serialize_block(tree, block_id);

    // Assume the parent-level data (e.g. slot) is in the top-level JSON.
    let parent_slot = nested_json.get("parent_slot").and_then(|v| v.as_i64());
    let block_slot = nested_json.get("slot").cloned();
    let block_time = nested_json.get("block_time").cloned();
    let total_base_fees = nested_json.get("total_base_fees").cloned();
    let total_priority_fees = nested_json.get("total_priority_fees").cloned();
    let total_tips = nested_json.get("total_tips").cloned();


    let mut transactions_json = Vec::new();

    // The block's "children" array is assumed to be a list of transactions.
    if let Some(transactions_array) = nested_json.get("children").and_then(|v| v.as_array()) {
        for transaction in transactions_array.iter() {
            // Extract transaction-level fields.
            let tx_signature = transaction.get("signature").cloned();
            let tx_failed = transaction.get("failed").cloned();
            let tx_tags = transaction.get("tags").cloned();
            let tx_id = transaction.get("id").and_then(|v| v.as_i64());

            // Flatten instructions within this transaction.
            let mut instructions = Vec::new();
            if let Some(instr_array) = transaction.get("children").and_then(|v| v.as_array()) {
                // Here, for the instructions we use the transaction id as the parent.
                flatten_children(block_slot.as_ref().and_then(|v| v.as_i64()), tx_id, None,instr_array, 0, String::new(), &mut instructions);
            }

            // Build a transaction JSON object that includes the flattened instructions.
            let tx_obj = json!({
                "type": transaction.get("type"),
                "signature": tx_signature,
                "failed": tx_failed,
                "tags": tx_tags,
                "id": tx_id,
                "instructions": instructions,
            });

            transactions_json.push(tx_obj);
        }
    }

    // Create the final block JSON object.
    let block_obj = json!({
        "type": "block",
        "slot": block_slot,
        "parent_slot": parent_slot,
        "block_time": block_time,
        "total_base_fees": total_base_fees,
        "total_priority_fees": total_priority_fees,
        "total_tips": total_tips,
        "transactions": transactions_json,
    });

    block_obj
}
