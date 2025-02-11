use classifier_core::ClassifiableTransaction;
use macros::declare_anchor_actions;
use solana_sdk::pubkey::Pubkey;

use crate::{util::find_transfer, ActionNodeId, ActionTrait, ActionTree, DexSwap};

declare_anchor_actions!(
    whirlpools,
    Swap {
        Args: {
            amount,
            a_to_b,
        },
        Accounts: {
            whirlpool,
            token_owner_account_a,
            token_owner_account_b,
            token_vault_a,
            token_vault_b,
        },
    },
    SwapV2 {
        Args: {
            amount,
            a_to_b,
        },
        Accounts: {
            whirlpool,
            token_owner_account_a,
            token_owner_account_b,
            token_vault_a,
            token_vault_b,
        },
    },
    TwoHopSwap {},
    TwoHopSwapV2 {},
);

impl ActionTrait for WhirlpoolsAction {
    fn recurse_during_classify(&self) -> bool {
        match self {
            WhirlpoolsAction::Swap(_) => true,
            WhirlpoolsAction::SwapV2(_) => true,
            _ => false,
        }
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
        action_id: ActionNodeId,
        tree: &ActionTree,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            WhirlpoolsAction::Swap(action) => {
                let (input_account, output_account, input_pool_account, output_pool_account) =
                    if action.a_to_b {
                        (
                            &action.token_owner_account_a,
                            &action.token_owner_account_b,
                            &action.token_vault_a,
                            &action.token_vault_b,
                        )
                    } else {
                        (
                            &action.token_owner_account_b,
                            &action.token_owner_account_a,
                            &action.token_vault_b,
                            &action.token_vault_a,
                        )
                    };

                whirlpool_into_dex_swap(
                    txn,
                    tree,
                    action_id,
                    *input_account,
                    *input_pool_account,
                    *output_account,
                    *output_pool_account,
                )?
            }
            WhirlpoolsAction::SwapV2(action) => {
                let (input_account, output_account, input_pool_account, output_pool_account) =
                    if action.a_to_b {
                        (
                            &action.token_owner_account_a,
                            &action.token_owner_account_b,
                            &action.token_vault_a,
                            &action.token_vault_b,
                        )
                    } else {
                        (
                            &action.token_owner_account_b,
                            &action.token_owner_account_a,
                            &action.token_vault_b,
                            &action.token_vault_a,
                        )
                    };

                whirlpool_into_dex_swap(
                    txn,
                    tree,
                    action_id,
                    *input_account,
                    *input_pool_account,
                    *output_account,
                    *output_pool_account,
                )?
            }
            _ => return Ok(None),
        };

        Ok(Some(dex_swap))
    }
}

fn whirlpool_into_dex_swap(
    txn: &ClassifiableTransaction,
    tree: &ActionTree,
    action_id: ActionNodeId,
    input_token_account: Pubkey,
    input_pool_account: Pubkey,
    output_token_account: Pubkey,
    output_pool_account: Pubkey,
) -> Result<DexSwap, anyhow::Error> {

	println!("IN WHIRLPOOL INTO DEX SWAP");
	
	let input_transfer = find_transfer(tree, action_id, &input_token_account, &input_pool_account)
			.ok_or_else(|| anyhow::anyhow!("No input transfer found"))?;

	println!("INPUT TRANSFER: {:?}", input_transfer);

	let output_transfer =
			find_transfer(tree, action_id, &output_pool_account, &output_token_account)
					.ok_or_else(|| anyhow::anyhow!("No output transfer found"))?;

	println!("OUTPUT TRANSFER: {:?}", output_transfer);

	let input_mint = txn.get_mint_for_token_account(&input_token_account)?;

	let output_mint = txn.get_mint_for_token_account(&output_token_account)?;

	Ok(DexSwap {
			input_mint,
			output_mint,
			input_token_account,
			output_token_account,
			input_amount: input_transfer.amount,
			output_amount: output_transfer.amount,
	})
}
