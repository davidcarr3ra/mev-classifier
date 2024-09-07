use std::collections::HashMap;

use action_tree::{ActionNodeId, ActionTree};
use actions::{Action, AssociatedToken, Token};
use classifier_core::ClassifiableTransaction;
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

use crate::instruction::{classify_instruction, ClassifyInstructionError};

#[derive(Debug, Error)]
pub enum ClassifyError {
    #[error(transparent)]
    ClassifyInstructionError(#[from] ClassifyInstructionError),
}

type Result<T> = std::result::Result<T, ClassifyError>;

pub fn classify_transaction(
    txn: &ClassifiableTransaction,
    tree: &mut ActionTree,
    transaction_id: ActionNodeId,
) -> Result<()> {
    let mut idx = 0;

    while idx < txn.instructions.len() {
        let indexes_used = classify_instruction(&txn, idx, tree, transaction_id)?;
        idx += indexes_used;
    }

    // Update transaction with created token accounts
    let created_tokens = find_created_token_accounts(tree, transaction_id);
    let mut_txn = match tree.get_mut(transaction_id).unwrap().get_mut() {
        Action::ClassifiableTransaction(txn) => txn,
        _ => unreachable!(),
    };

    if !created_tokens.is_empty() {
        mut_txn.created_tokens = Some(created_tokens);
    }

    Ok(())
}

fn find_created_token_accounts(
    tree: &ActionTree,
    transaction_id: ActionNodeId,
) -> HashMap<Pubkey, Pubkey> {
    // Identify token accounts created/destroyed within the transaction.
    // These are not included in pre/post token account balances sent by the RPC.
    let mut created_tokens = HashMap::new();

    for node_id in tree.descendants(transaction_id) {
        let node = tree.get(node_id).unwrap().get();

        match node {
            Action::AssociatedToken(AssociatedToken::Create(ix)) => {
                created_tokens.insert(ix.associated_token_address, ix.mint);
            }
            Action::AssociatedToken(AssociatedToken::CreateIdempotent(ix)) => {
                created_tokens.insert(ix.associated_token_address, ix.mint);
            }
            Action::Token(Token::InitializeAccount(ix)) => {
                created_tokens.insert(ix.account, ix.mint);
            }
            Action::Token(Token::InitializeAccount2(ix)) => {
                created_tokens.insert(ix.account, ix.mint);
            }
            Action::Token(Token::InitializeAccount3(ix)) => {
                created_tokens.insert(ix.account, ix.mint);
            }
            _ => continue,
        }
    }

    created_tokens
}
