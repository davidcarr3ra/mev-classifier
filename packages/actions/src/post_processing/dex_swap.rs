use classifier_core::ClassifiableTransaction;
use macros::action;
use solana_sdk::pubkey::Pubkey;

use crate::{Action, ActionNodeId, ActionTrait, ActionTree};

#[action]
pub struct DexSwap {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,

    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,

    pub input_amount: u64,
    pub output_amount: u64,
}

impl DexSwap {
    pub fn try_from(
        action: &Action,
        txn: &ClassifiableTransaction,
        action_id: ActionNodeId,
        tree: &ActionTree,
    ) -> Option<Self> {
        match action.into_dex_swap(txn, action_id, tree) {
            Ok(dex_swap) => dex_swap,
            Err(e) => {
                tracing::error!(
                    "Failed to convert action into DexSwap. \n Action: {:?} \n Error: {:?} \n Signature: {:?}",
                    action,
                    e,
                    txn.signature
                );
                None
            }
        }
    }
}

impl ActionTrait for DexSwap {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("DexSwap should not be classified");
    }

    fn serializable(&self) -> bool {
        true
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "dexSwap",
            "inputMint": self.input_mint.to_string(),
            "outputMint": self.output_mint.to_string(),
            "inputTokenAccount": self.input_token_account.to_string(),
            "outputTokenAccount": self.output_token_account.to_string(),
            "inputAmount": self.input_amount,
            "outputAmount": self.output_amount,
        })
    }
}
