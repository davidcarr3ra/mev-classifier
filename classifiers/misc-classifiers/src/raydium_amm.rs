use actions::{
    raydium_amm_actions::{SwapBaseIn, SwapBaseOut},
    Action, RaydiumAmmAction,
};
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::pubkey::Pubkey;

pub struct RaydiumAmmClassifier;

impl InstructionClassifier for RaydiumAmmClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let (&tag, rest) = ix
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
            9 => classify_swap_base_in(txn, ix, rest),
            10 => Ok(Some(Action::RaydiumAmmAction(
                RaydiumAmmAction::Preinitialize,
            ))),
            11 => classify_swap_base_out(txn, ix, rest),
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

fn classify_swap_base_in(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    rest: &[u8],
) -> ClassifyInstructionResult {
    let amount_in = rest
        .get(0..8)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or_else(|| anyhow::anyhow!("Invalid Raydium AMM swap base in instruction"))?;

    if ix.accounts.len() < 17 {
        return Err(anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: not enough accounts"
        ));
    }

    let user_source_account = txn.get_pubkey(ix.accounts[15]).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: user source account not found"
        )
    })?;

    let user_destination_account = txn.get_pubkey(ix.accounts[16]).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: user destination account not found"
        )
    })?;

    Ok(Some(Action::RaydiumAmmAction(
        RaydiumAmmAction::SwapBaseIn(SwapBaseIn {
            amount_in,
            user_destination_account,
            user_source_account,
        }),
    )))
}

fn classify_swap_base_out(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    rest: &[u8],
) -> ClassifyInstructionResult {
    let amount_in = rest
        .get(0..8)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or_else(|| anyhow::anyhow!("Invalid Raydium AMM swap base in instruction"))?;

    if ix.accounts.len() < 17 {
        return Err(anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: not enough accounts"
        ));
    }

    let user_source_account = txn.get_pubkey(ix.accounts[15]).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: user source account not found"
        )
    })?;

    let user_destination_account = txn.get_pubkey(ix.accounts[16]).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid Raydium AMM swap base in instruction: user destination account not found"
        )
    })?;

    Ok(Some(Action::RaydiumAmmAction(
        RaydiumAmmAction::SwapBaseOut(SwapBaseOut {
            amount_in,
            user_destination_account,
            user_source_account,
        }),
    )))
}
