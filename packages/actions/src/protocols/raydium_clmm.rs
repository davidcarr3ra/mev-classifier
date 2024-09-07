use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{ActionTrait, DexSwap};

declare_anchor_actions!(
    raydium_clmm,
    Swap {
        Args: {
            amount,
            is_base_input,
        },
        Accounts: {
            pool_state,
            input_token_account,
            output_token_account,
        }
    }
);

impl ActionTrait for RaydiumClmmAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            RaydiumClmmAction::Swap(action) => DexSwap {
                input_mint: txn.get_mint_for_token_account(&action.input_token_account)?,
                output_mint: txn.get_mint_for_token_account(&action.output_token_account)?,
                input_token_account: action.input_token_account,
                output_token_account: action.output_token_account,
            },
        };

        Ok(Some(dex_swap))
    }
}
