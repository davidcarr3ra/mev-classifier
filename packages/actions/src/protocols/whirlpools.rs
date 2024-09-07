use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{ActionTrait, DexSwap};

declare_anchor_actions!(
    whirlpools,
    Swap {
        Args: {
            amount,
        },
        Accounts: {
            whirlpool,
            token_owner_account_a,
            token_owner_account_b,
        },
    },
    SwapV2 {
        Args: {
            amount,
        },
        Accounts: {
            whirlpool,
            token_owner_account_a,
            token_owner_account_b,
        },
    },
    TwoHopSwap {},
    TwoHopSwapV2 {},
);

impl ActionTrait for WhirlpoolsAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            WhirlpoolsAction::Swap(action) => {
                let input_mint = txn.get_mint_for_token_account(&action.token_owner_account_a)?;
                let output_mint = txn.get_mint_for_token_account(&action.token_owner_account_b)?;

                DexSwap {
                    input_mint,
                    output_mint,
                    input_token_account: action.token_owner_account_a,
                    output_token_account: action.token_owner_account_b,
                }
            }
            WhirlpoolsAction::SwapV2(action) => {
                let input_mint = txn.get_mint_for_token_account(&action.token_owner_account_a)?;
                let output_mint = txn.get_mint_for_token_account(&action.token_owner_account_b)?;

                DexSwap {
                    input_mint,
                    output_mint,
                    input_token_account: action.token_owner_account_a,
                    output_token_account: action.token_owner_account_b,
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(dex_swap))
    }
}
