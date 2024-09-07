use classifier_core::ClassifiableTransaction;
use macros::action;
use solana_sdk::pubkey::Pubkey;

use crate::{Action, ActionTrait};

#[action]
pub struct DexSwap {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,

    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
}

impl DexSwap {
    pub fn try_from(action: &Action, txn: &ClassifiableTransaction) -> Option<Self> {
        match action.into_dex_swap(txn) {
            Ok(dex_swap) => dex_swap,
            Err(e) => {
                tracing::error!("Failed to convert action into DexSwap: {:?}", e);
                None
            }
        }
    }
}

impl ActionTrait for DexSwap {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("DexSwap should not be classified");
    }
}
