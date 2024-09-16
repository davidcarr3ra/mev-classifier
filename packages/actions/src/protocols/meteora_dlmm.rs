use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;

use crate::{util::find_transfer, ActionNodeId, ActionTrait, ActionTree, DexSwap};

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
            reserve_x,
            reserve_y,
            token_x_mint,
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
            reserve_x,
            reserve_y,
            token_x_mint,
        }
    }
);

impl ActionTrait for MeteoraDlmmAction {
    fn recurse_during_classify(&self) -> bool {
        match self {
            MeteoraDlmmAction::Swap(_) | MeteoraDlmmAction::SwapExactOut(_) => true,
        }
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
        action_id: ActionNodeId,
        tree: &ActionTree,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            MeteoraDlmmAction::Swap(action) => {
                let input_mint = txn.get_mint_for_token_account(&action.user_token_in)?;
                let x_for_y = input_mint == action.token_x_mint;

                let output_amount = if x_for_y {
                    find_transfer(tree, action_id, &action.reserve_y, &action.user_token_out)
                        .ok_or_else(|| anyhow::anyhow!("Could not find transfer"))?
                        .amount
                } else {
                    find_transfer(tree, action_id, &action.reserve_x, &action.user_token_out)
                        .ok_or_else(|| anyhow::anyhow!("Could not find transfer"))?
                        .amount
                };

                DexSwap {
                    input_mint,
                    output_mint: txn.get_mint_for_token_account(&action.user_token_out)?,
                    input_token_account: action.user_token_in,
                    output_token_account: action.user_token_out,
                    input_amount: action.amount_in,
                    output_amount,
                }
            }
            MeteoraDlmmAction::SwapExactOut(action) => {
                let input_mint = txn.get_mint_for_token_account(&action.user_token_in)?;
                let x_for_y = input_mint == action.token_x_mint;

                let input_amount = if x_for_y {
                    find_transfer(tree, action_id, &action.user_token_in, &action.reserve_x)
                        .ok_or_else(|| anyhow::anyhow!("Could not find transfer"))?
                        .amount
                } else {
                    find_transfer(tree, action_id, &action.user_token_in, &action.reserve_y)
                        .ok_or_else(|| anyhow::anyhow!("Could not find transfer"))?
                        .amount
                };

                DexSwap {
                    input_mint: txn.get_mint_for_token_account(&action.user_token_in)?,
                    output_mint: txn.get_mint_for_token_account(&action.user_token_out)?,
                    input_token_account: action.user_token_in,
                    output_token_account: action.user_token_out,
                    input_amount,
                    output_amount: action.out_amount,
                }
            }
        };

        Ok(Some(dex_swap))
    }
}
