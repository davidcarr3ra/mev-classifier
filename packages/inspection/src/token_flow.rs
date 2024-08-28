use std::str::FromStr;

use base58::FromBase58;
use solana_sdk::{program_error::ProgramError, pubkey::Pubkey};
use solana_transaction_status::{
    EncodedTransaction, TransactionBinaryEncoding, TransactionStatusMeta, UiCompiledInstruction,
    UiConfirmedBlock, UiMessage, UiRawMessage, UiTransaction, UiTransactionStatusMeta,
};
use spl_token::instruction::TokenInstruction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenFlowError {
    #[error("No transaction in block data")]
    MissingTransactions,

    #[error("No signatures in block data")]
    MissingSignatures,

    #[error("Failed to decode transaction")]
    DecodeError,

    #[error("Failed to decode token instruction")]
    DecodeTokenInstructionError(#[source] ProgramError),
}

type Result<T> = std::result::Result<T, TokenFlowError>;

pub struct TokenFlow {}

pub fn token_flows_from_ui_block(ui_block: UiConfirmedBlock) -> Result<()> {
    let txns = match ui_block.transactions {
        None => return Err(TokenFlowError::MissingTransactions),
        Some(txns) => txns,
    };

    for (idx, txn) in txns.iter().enumerate() {
        let meta = match &txn.meta {
            None => {
                eprintln!("No meta in transaction {}", idx);
                continue;
            }
            Some(meta) => meta,
        };

        let v_txn = match txn.transaction.decode() {
            Some(v_txn) => v_txn,
            None => {
                eprintln!("Failed to decode transaction {}", idx);
                continue;
            }
        };

        let signature = match v_txn.signatures.first() {
            Some(signature) => signature,
            None => {
                eprintln!("No signatures in transaction {}", idx);
                continue;
            }
        };

        if signature.to_string() == "3eecnAy7vgcMiy9WEUsmxKjshYsJSapUiciDbbu4ZJm3K5kP4z3i3YpqcLhyvDY8tTBhqKa1DryDjeFe1WsBkeqc" {
            identify_atomic_arbitrage(meta);
        }

        println!("signature: {}", signature);
    }

    Ok(())
}

fn identify_atomic_arbitrage(meta: &UiTransactionStatusMeta) {
    println!("pre_balances: {:?}", meta.pre_balances);
    println!("pre_token_balances: {:?}", meta.pre_token_balances);
    println!("post_balances: {:?}", meta.post_balances);
    println!("post_token_balances: {:?}", meta.post_token_balances);
}
