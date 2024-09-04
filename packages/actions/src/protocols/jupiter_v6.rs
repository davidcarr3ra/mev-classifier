use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{ActionTrait, DexSwap};

declare_anchor_actions!(
    jupiter_v6,
    Route {
        Args: {
            in_amount,
            quoted_out_amount,
            slippage_bps,
        },
        Accounts: {
            user_source_token_account,
            user_destination_token_account,
        },
    },
);

impl ActionTrait for JupiterV6Action {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}

impl JupiterV6Action {
    pub fn into_dex_swap(&self, txn: &ClassifiableTransaction) -> Result<DexSwap, anyhow::Error> {
        match self {
            JupiterV6Action::Route(route) => route.into_dex_swap(txn),
        }
    }
}

impl jupiter_v6_actions::Route {
    pub fn into_dex_swap(&self, txn: &ClassifiableTransaction) -> Result<DexSwap, anyhow::Error> {
        let input_mint = txn.get_mint_for_token_account(&self.user_source_token_account)?;
        let output_mint = txn.get_mint_for_token_account(&self.user_destination_token_account)?;

        Ok(DexSwap {
            input_mint,
            output_mint,
            input_token_account: self.user_source_token_account,
            output_token_account: self.user_destination_token_account,
        })
    }
}
