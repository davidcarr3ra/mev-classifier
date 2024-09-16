use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{util::find_transfer, ActionNodeId, ActionTrait, ActionTree, DexSwap};

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
            input_vault,
            output_vault,
        }
    }
);

impl ActionTrait for RaydiumClmmAction {
    fn recurse_during_classify(&self) -> bool {
        match self {
            RaydiumClmmAction::Swap(_) => true,
        }
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
        action_id: ActionNodeId,
        tree: &ActionTree,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            RaydiumClmmAction::Swap(action) => {
                // Accounts needed to find output amount from token transfer
                let (from, to) = if action.is_base_input {
                    (&action.output_vault, &action.output_token_account)
                } else {
                    (&action.input_token_account, &action.input_vault)
                };

                let non_base_amount = find_transfer(tree, action_id, from, to)
                    .ok_or_else(|| anyhow::anyhow!("Could not find transfer"))?
                    .amount;

                let (input_amount, output_amount) = if action.is_base_input {
                    (action.amount, non_base_amount)
                } else {
                    (non_base_amount, action.amount)
                };

                DexSwap {
                    input_mint: txn.get_mint_for_token_account(&action.input_token_account)?,
                    output_mint: txn.get_mint_for_token_account(&action.output_token_account)?,
                    input_token_account: action.input_token_account,
                    output_token_account: action.output_token_account,
                    input_amount,
                    output_amount,
                }
            }
        };

        Ok(Some(dex_swap))
    }
}
