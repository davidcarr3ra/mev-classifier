use classifier_core::ClassifiableTransaction;
use macros::action_enum;

use crate::{ActionNodeId, ActionTrait, ActionTree, DexSwap};

#[action_enum]
pub enum RaydiumAmmAction {
    Initialize,
    Initialize2,
    Preinitialize,
    MonitorStep,
    Deposit,
    Withdraw,
    MigrateToOpenBook,
    SetParams,
    WithdrawPnl,
    WithdrawSrm,
    SwapBaseIn(raydium_amm_actions::SwapBaseIn),
    SwapBaseOut(raydium_amm_actions::SwapBaseOut),
    SimulateInstruction,
    AdminCancelOrders,
    CreateConfigAccount,
    UpdateConfigAccount,
}

impl ActionTrait for RaydiumAmmAction {
    fn recurse_during_classify(&self) -> bool {
        match self {
            RaydiumAmmAction::SwapBaseIn(_) | RaydiumAmmAction::SwapBaseOut(_) => true,
            _ => false,
        }
    }

    fn into_dex_swap(
        &self,
        txn: &ClassifiableTransaction,
        _action_id: ActionNodeId,
        _tree: &ActionTree,
    ) -> Result<Option<DexSwap>, anyhow::Error> {
        let dex_swap = match self {
            RaydiumAmmAction::SwapBaseIn(action) => DexSwap {
                input_mint: txn.get_mint_for_token_account(&action.user_source_account)?,
                output_mint: txn.get_mint_for_token_account(&action.user_destination_account)?,
                input_token_account: action.user_source_account,
                output_token_account: action.user_destination_account,
                input_amount: action.amount_in,
                output_amount: 0,
            },
            RaydiumAmmAction::SwapBaseOut(action) => DexSwap {
                input_mint: txn.get_mint_for_token_account(&action.user_source_account)?,
                output_mint: txn.get_mint_for_token_account(&action.user_destination_account)?,
                input_token_account: action.user_source_account,
                output_token_account: action.user_destination_account,
                input_amount: action.amount_in,
                output_amount: 0,
            },
            _ => return Err(anyhow::anyhow!("Invalid Raydium AMM action")),
        };

        Ok(Some(dex_swap))
    }
}

pub mod raydium_amm_actions {
    use macros::action;
    use solana_sdk::pubkey::Pubkey;

    #[action]
    pub struct SwapBaseIn {
        pub amount_in: u64,
        pub user_source_account: Pubkey,
        pub user_destination_account: Pubkey,
    }

    #[action]
    pub struct SwapBaseOut {
        pub amount_in: u64,
        pub user_source_account: Pubkey,
        pub user_destination_account: Pubkey,
    }
}
