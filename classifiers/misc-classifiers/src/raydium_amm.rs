use actions::{Action, RaydiumAmmAction};
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::pubkey::Pubkey;

pub struct RaydiumAmmClassifier;

impl InstructionClassifier for RaydiumAmmClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let (&tag, _rest) = ix
            .data
            .split_first()
            .ok_or_else(|| anyhow::anyhow!("Invalid Raydium AMM instruction data"))?;

        match tag {
            0 => Ok(Some(Action::RaydiumAmmAction(RaydiumAmmAction::Initialize))),
            1 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::Initialize2,
            ))),
            2 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::MonitorStep,
            ))),
            3 => Ok(Some(Action::RaydiumAmmAction(RaydiumAmmAction::Deposit))),
            4 => Ok(Some(Action::RaydiumAmmAction(RaydiumAmmAction::Withdraw))),
            5 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::MigrateToOpenBook,
            ))),
            6 => Ok(Some(Action::RaydiumAmmAction(RaydiumAmmAction::SetParams))),
            7 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::WithdrawPnl,
            ))),
            8 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::WithdrawSrm,
            ))),
            9 => Ok(Some(Action::RaydiumAmmAction(RaydiumAmmAction::SwapBaseIn))),
            10 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::Preinitialize,
            ))),
            11 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::SwapBaseOut,
            ))),
            12 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::SimulateInstruction,
            ))),
            13 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::AdminCancelOrders,
            ))),
            14 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::CreateConfigAccount,
            ))),
            15 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::UpdateConfigAccount,
            ))),
            _ => Ok(None),
        }
    }
}
