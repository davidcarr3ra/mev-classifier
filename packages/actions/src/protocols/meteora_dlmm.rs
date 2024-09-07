use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{ActionTrait, DexSwap};

declare_anchor_actions!(
    meteora_dlmm,
    Swap {
        Args: {
            amount_in,
        },
        Accounts: {
            lb_pair,
            user_token_in,
            user_token_out,
        },
    },
    SwapExactOut {
        Args: {
            out_amount,
        },
        Accounts: {
            lb_pair,
            user_token_in,
            user_token_out,
        }
    }
);

impl ActionTrait for MeteoraDlmmAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            MeteoraDlmmAction::Swap(action) => DexSwap {
                input_mint: txn.get_mint_for_token_account(&action.user_token_in)?,
                output_mint: txn.get_mint_for_token_account(&action.user_token_out)?,
                input_token_account: action.user_token_in,
                output_token_account: action.user_token_out,
            },
            MeteoraDlmmAction::SwapExactOut(action) => DexSwap {
                input_mint: txn.get_mint_for_token_account(&action.user_token_in)?,
                output_mint: txn.get_mint_for_token_account(&action.user_token_out)?,
                input_token_account: action.user_token_in,
                output_token_account: action.user_token_out,
            },
        };
        
        Ok(Some(dex_swap))
    }
}
