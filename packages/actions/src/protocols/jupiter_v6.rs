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
    RouteWithTokenLedger {
        Args: {
            quoted_out_amount,
            slippage_bps,
        },
        Accounts: {
            user_source_token_account,
            user_destination_token_account,
        },
    },
    SharedAccountsRoute {
        Args: {
            in_amount,
            quoted_out_amount,
            slippage_bps,
        },
        Accounts: {
            source_token_account,
            destination_token_account,
            program_source_token_account,
            program_destination_token_account,
        },
    }
);

impl ActionTrait for JupiterV6Action {
    fn recurse_during_classify(&self) -> bool {
        true
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        match self {
            JupiterV6Action::Route(route) => route.into_dex_swap(txn),
            JupiterV6Action::RouteWithTokenLedger(route) => route.into_dex_swap(txn),
            JupiterV6Action::SharedAccountsRoute(route) => route.into_dex_swap(txn),
        }
    }
}

impl jupiter_v6_actions::Route {
    pub fn into_dex_swap(&self, txn: &ClassifiableTransaction) -> Result<Option<DexSwap>, anyhow::Error> {
        let input_mint = txn.get_mint_for_token_account(&self.user_source_token_account)?;
        let output_mint = txn.get_mint_for_token_account(&self.user_destination_token_account)?;

        Ok(Some(DexSwap {
            input_mint,
            output_mint,
            input_token_account: self.user_source_token_account,
            output_token_account: self.user_destination_token_account,
        }))
    }
}

impl jupiter_v6_actions::RouteWithTokenLedger {
    pub fn into_dex_swap(&self, txn: &ClassifiableTransaction) -> Result<Option<DexSwap>, anyhow::Error> {
        let input_mint = txn.get_mint_for_token_account(&self.user_source_token_account)?;
        let output_mint = txn.get_mint_for_token_account(&self.user_destination_token_account)?;

        Ok(Some(DexSwap {
            input_mint,
            output_mint,
            input_token_account: self.user_source_token_account,
            output_token_account: self.user_destination_token_account,
        }))
    }
}

impl jupiter_v6_actions::SharedAccountsRoute {
    pub fn into_dex_swap(&self, txn: &ClassifiableTransaction) -> Result<Option<DexSwap>, anyhow::Error> {
        let input_mint = txn.get_mint_for_token_account(&self.program_source_token_account)?;
        let output_mint =
            txn.get_mint_for_token_account(&self.program_destination_token_account)?;

        Ok(Some(DexSwap {
            input_mint,
            output_mint,
            input_token_account: self.source_token_account,
            output_token_account: self.destination_token_account,
        }))
    }
}
