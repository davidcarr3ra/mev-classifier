use actions::AssociatedToken;
use borsh::BorshDeserialize;
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::instruction::AssociatedTokenAccountInstruction;

pub struct AssociatedTokenClassifier;

impl InstructionClassifier for AssociatedTokenClassifier {
    const ID: Pubkey = spl_associated_token_account::id();

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        if ix.data.is_empty() {
            return classify_create(txn, ix)
        }
        
        let instruction =
            AssociatedTokenAccountInstruction::try_from_slice(&ix.data).map_err(|_| {
                anyhow::anyhow!("Failed to deserialize associated token account instruction")
            })?;

        match instruction {
            AssociatedTokenAccountInstruction::RecoverNested => {
                Ok(Some(AssociatedToken::RecoverNested.into()))
            }
            AssociatedTokenAccountInstruction::Create => classify_create(txn, ix),
            AssociatedTokenAccountInstruction::CreateIdempotent => {
                classify_create_idempotent(txn, ix)
            }
        }
    }
}

fn classify_create(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    if ix.accounts.len() < 6 {
        return Err(anyhow::anyhow!(
            "Instruction accounts length is less than 5 in associated token create"
        ));
    }

    let payer = txn.get_pubkey(ix.accounts[0]).ok_or_else(|| {
        anyhow::anyhow!("Failed to get pubkey for account at index 0 in associated token create")
    })?;

    let associated_token_address = txn.get_pubkey(ix.accounts[1]).ok_or_else(|| {
        anyhow::anyhow!("Failed to get pubkey for account at index 1 in associated token create")
    })?;

    let wallet = txn.get_pubkey(ix.accounts[2]).ok_or_else(|| {
        anyhow::anyhow!("Failed to get pubkey for account at index 2 in associated token create")
    })?;

    let mint = txn.get_pubkey(ix.accounts[3]).ok_or_else(|| {
        anyhow::anyhow!("Failed to get pubkey for account at index 3 in associated token create")
    })?;

    let token_program = txn.get_pubkey(ix.accounts[5]).ok_or_else(|| {
        anyhow::anyhow!("Failed to get pubkey for account at index 4 in associated token create")
    })?;

    let create = AssociatedToken::Create(actions::associated_token_actions::Create {
        payer,
        associated_token_address,
        wallet,
        mint,
        token_program,
    });

    Ok(Some(create.into()))
}

fn classify_create_idempotent(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    if ix.accounts.len() < 6 {
        return Err(anyhow::anyhow!(
            "Instruction accounts length is less than 5 in associated token create idempotent"
        ));
    }

    let payer = txn.get_pubkey(ix.accounts[0]).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get pubkey for account at index 0 in associated token create idempotent"
        )
    })?;

    let associated_token_address = txn.get_pubkey(ix.accounts[1]).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get pubkey for account at index 1 in associated token create idempotent"
        )
    })?;

    let wallet = txn.get_pubkey(ix.accounts[2]).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get pubkey for account at index 2 in associated token create idempotent"
        )
    })?;

    let mint = txn.get_pubkey(ix.accounts[3]).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get pubkey for account at index 3 in associated token create idempotent"
        )
    })?;

    let token_program = txn.get_pubkey(ix.accounts[5]).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get pubkey for account at index 4 in associated token create idempotent"
        )
    })?;

    let create_idempotent =
        AssociatedToken::CreateIdempotent(actions::associated_token_actions::CreateIdempotent {
            payer,
            associated_token_address,
            wallet,
            mint,
            token_program,
        });

    Ok(Some(create_idempotent.into()))
}
