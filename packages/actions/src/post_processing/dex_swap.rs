use classifier_core::ClassifiableTransaction;
use macros::action;
use solana_sdk::pubkey::Pubkey;

use crate::{Action, ActionTrait};

#[action]
pub struct DexSwap {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
}

impl DexSwap {
    pub fn try_from(action: &Action, txn: &ClassifiableTransaction) -> Option<Self> {
        let dex_swap = match action {
            Action::JupiterV6Action(action) => action.into_dex_swap(txn),
            _ => return None,
        };

        if let Ok(dex_swap) = dex_swap {
            Some(dex_swap)
        } else {
            None
        }
    }
}

impl ActionTrait for DexSwap {
    fn recurse_during_classify(&self) -> bool {
        unreachable!("DexSwap should not be classified");
    }
}
