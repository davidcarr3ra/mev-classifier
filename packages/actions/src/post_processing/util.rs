use action_tree::ActionNodeId;
use solana_sdk::pubkey::Pubkey;

use crate::{Action, ActionTree, Token};

use super::TokenTransfer;

pub fn find_transfer(
    tree: &ActionTree,
    node_id: ActionNodeId,
    from: &Pubkey,
    to: &Pubkey,
) -> Option<TokenTransfer> {
    for child_id in tree.children(node_id) {
        let action = tree.get(child_id).unwrap().get();

				// TODO: add token2022 (token program extension)
        match action {
            Action::Token(token_action) => match token_action {
                Token::Transfer(transfer) => {
                    if transfer.source == *from && transfer.destination == *to {
                        return Some(TokenTransfer {
                            source: transfer.source,
                            destination: transfer.destination,
                            amount: transfer.amount,
                        });
                    }
                }
                Token::TransferChecked(transfer) => {
                    if transfer.source == *from && transfer.destination == *to {
                        return Some(TokenTransfer {
                            source: transfer.source,
                            destination: transfer.destination,
                            amount: transfer.amount,
                        });
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    None
}
