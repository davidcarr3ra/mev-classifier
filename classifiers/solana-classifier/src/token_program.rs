use actions::{token_actions, Token};
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::{program_option::COption, pubkey::Pubkey};
use spl_token::instruction::{AuthorityType, TokenInstruction};
use anchor_spl::token_2022;
use std::marker::PhantomData;

pub trait TokenProgramId {
	const ID: Pubkey;
}

// Marker types for the two programs
pub struct OriginalToken;
pub struct Token2022;

impl TokenProgramId for OriginalToken {
	const ID: Pubkey = spl_token::ID;
}

impl TokenProgramId for Token2022 {
	const ID: Pubkey = token_2022::ID;
}

// Generic classifier that uses the marker type
pub struct GenericTokenProgramClassifier<T: TokenProgramId> {
	phantom: PhantomData<T>,
}

impl<T: TokenProgramId> InstructionClassifier for GenericTokenProgramClassifier<T> {
    const ID: Pubkey = T::ID;

    fn classify_instruction(
        txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        let token_instruction = TokenInstruction::unpack(&ix.data)
            .map_err(|e| anyhow::anyhow!("Failed to unpack token instruction: {:?}", e))?;

        match token_instruction {
            TokenInstruction::InitializeMint { .. } => classify_initialize_mint(txn, ix),
            TokenInstruction::InitializeAccount => classify_initialize_account(txn, ix),
            TokenInstruction::InitializeMultisig { .. } => classify_initialize_multisig(txn, ix),
            TokenInstruction::Transfer { amount } => classify_transfer(txn, ix, amount),
            TokenInstruction::Approve { amount } => classify_approve(txn, ix, amount),
            TokenInstruction::Revoke => classify_revoke(txn, ix),
            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => classify_set_authority(txn, ix, authority_type, new_authority),
            TokenInstruction::MintTo { amount } => classify_mint_to(txn, ix, amount),
            TokenInstruction::Burn { amount } => classify_burn(txn, ix, amount),
            TokenInstruction::CloseAccount => classify_close_account(txn, ix),
            TokenInstruction::FreezeAccount => classify_freeze_account(txn, ix),
            TokenInstruction::ThawAccount => classify_thaw_account(txn, ix),
            TokenInstruction::TransferChecked { amount, decimals } => {
                classify_transfer_checked(txn, ix, amount, decimals)
            }
            TokenInstruction::ApproveChecked { amount, decimals } => {
                classify_approve_checked(txn, ix, amount, decimals)
            }
            TokenInstruction::MintToChecked { amount, decimals } => {
                classify_mint_to_checked(txn, ix, amount, decimals)
            }
            TokenInstruction::BurnChecked { amount, decimals } => {
                classify_burn_checked(txn, ix, amount, decimals)
            }
            TokenInstruction::InitializeAccount2 { owner } => {
                classify_initialize_account2(txn, ix, owner)
            }
            TokenInstruction::SyncNative => classify_sync_native(txn, ix),
            TokenInstruction::InitializeAccount3 { owner } => {
                classify_initialize_account3(txn, ix, owner)
            }
            TokenInstruction::InitializeMultisig2 { .. } => classify_initialize_multisig2(txn, ix),
            TokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => classify_initialize_mint2(
                txn,
                ix,
                decimals,
                mint_authority,
                freeze_authority.into(),
            ),
            TokenInstruction::GetAccountDataSize => classify_get_account_data_size(txn, ix),
            TokenInstruction::InitializeImmutableOwner => {
                classify_initialize_immutable_owner(txn, ix)
            }
            TokenInstruction::AmountToUiAmount { amount } => {
                classify_amount_to_ui_amount(txn, ix, amount)
            }
            TokenInstruction::UiAmountToAmount { ui_amount } => {
                classify_ui_amount_to_amount(txn, ix, ui_amount)
            }
        }
    }
}

// pub struct TokenProgramClassifier;

// impl InstructionClassifier for TokenProgramClassifier {
//     const ID: Pubkey = spl_token::ID;

//     fn classify_instruction(
//         txn: &ClassifiableTransaction,
//         ix: &ClassifiableInstruction,
//     ) -> ClassifyInstructionResult {
//         let token_instruction = TokenInstruction::unpack(&ix.data)
//             .map_err(|e| anyhow::anyhow!("Failed to unpack token instruction: {:?}", e))?;

//         match token_instruction {
//             TokenInstruction::InitializeMint { .. } => classify_initialize_mint(txn, ix),
//             TokenInstruction::InitializeAccount => classify_initialize_account(txn, ix),
//             TokenInstruction::InitializeMultisig { .. } => classify_initialize_multisig(txn, ix),
//             TokenInstruction::Transfer { amount } => classify_transfer(txn, ix, amount),
//             TokenInstruction::Approve { amount } => classify_approve(txn, ix, amount),
//             TokenInstruction::Revoke => classify_revoke(txn, ix),
//             TokenInstruction::SetAuthority {
//                 authority_type,
//                 new_authority,
//             } => classify_set_authority(txn, ix, authority_type, new_authority),
//             TokenInstruction::MintTo { amount } => classify_mint_to(txn, ix, amount),
//             TokenInstruction::Burn { amount } => classify_burn(txn, ix, amount),
//             TokenInstruction::CloseAccount => classify_close_account(txn, ix),
//             TokenInstruction::FreezeAccount => classify_freeze_account(txn, ix),
//             TokenInstruction::ThawAccount => classify_thaw_account(txn, ix),
//             TokenInstruction::TransferChecked { amount, decimals } => {
//                 classify_transfer_checked(txn, ix, amount, decimals)
//             }
//             TokenInstruction::ApproveChecked { amount, decimals } => {
//                 classify_approve_checked(txn, ix, amount, decimals)
//             }
//             TokenInstruction::MintToChecked { amount, decimals } => {
//                 classify_mint_to_checked(txn, ix, amount, decimals)
//             }
//             TokenInstruction::BurnChecked { amount, decimals } => {
//                 classify_burn_checked(txn, ix, amount, decimals)
//             }
//             TokenInstruction::InitializeAccount2 { owner } => {
//                 classify_initialize_account2(txn, ix, owner)
//             }
//             TokenInstruction::SyncNative => classify_sync_native(txn, ix),
//             TokenInstruction::InitializeAccount3 { owner } => {
//                 classify_initialize_account3(txn, ix, owner)
//             }
//             TokenInstruction::InitializeMultisig2 { .. } => classify_initialize_multisig2(txn, ix),
//             TokenInstruction::InitializeMint2 {
//                 decimals,
//                 mint_authority,
//                 freeze_authority,
//             } => classify_initialize_mint2(
//                 txn,
//                 ix,
//                 decimals,
//                 mint_authority,
//                 freeze_authority.into(),
//             ),
//             TokenInstruction::GetAccountDataSize => classify_get_account_data_size(txn, ix),
//             TokenInstruction::InitializeImmutableOwner => {
//                 classify_initialize_immutable_owner(txn, ix)
//             }
//             TokenInstruction::AmountToUiAmount { amount } => {
//                 classify_amount_to_ui_amount(txn, ix, amount)
//             }
//             TokenInstruction::UiAmountToAmount { ui_amount } => {
//                 classify_ui_amount_to_amount(txn, ix, ui_amount)
//             }
//         }
//     }
// }

fn check_account_len(ix: &ClassifiableInstruction, expected: usize) -> Result<(), anyhow::Error> {
    if ix.accounts.len() != expected {
        return Err(anyhow::anyhow!(
            "Expected {} accounts for instruction, found {}",
            expected,
            ix.accounts.len()
        ));
    }

    Ok(())
}

fn check_account_len_gte(
    ix: &ClassifiableInstruction,
    expected: usize,
) -> Result<(), anyhow::Error> {
    if ix.accounts.len() < expected {
        return Err(anyhow::anyhow!(
            "Expected >= than {} accounts for instruction, found {}",
            expected,
            ix.accounts.len()
        ));
    }

    Ok(())
}

fn classify_initialize_mint(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len(ix, 2)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeMint(token_actions::InitializeMint { mint }).into(),
    ))
}

fn classify_initialize_account(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len(ix, 4)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeAccount(token_actions::InitializeAccount {
            account,
            mint,
            owner,
        })
        .into(),
    ))
}

fn classify_initialize_multisig(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 1)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeMultisig(token_actions::InitializeMultisig { account }).into(),
    ))
}

fn classify_transfer(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let source = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let destination = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::Transfer(token_actions::Transfer {
            source,
            destination,
            owner,
            amount,
        })
        .into(),
    ))
}

fn classify_approve(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let source = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let delegate = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::Approve(token_actions::Approve {
            source,
            delegate,
            owner,
            amount,
        })
        .into(),
    ))
}

fn classify_revoke(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 2)?;

    let source = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let owner = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::Revoke(token_actions::Revoke { source, owner }).into(),
    ))
}

fn classify_set_authority(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    authority_type: AuthorityType,
    new_authority: COption<Pubkey>,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 2)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let current_authority = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::SetAuthority(token_actions::SetAuthority {
            account,
            current_authority,
            authority_type: authority_type as u8,
            new_authority: new_authority.into(),
        })
        .into(),
    ))
}

fn classify_mint_to(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let account = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint_authority = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::MintTo(token_actions::MintTo {
            mint,
            account,
            mint_authority,
            amount,
        })
        .into(),
    ))
}

fn classify_burn(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::Burn(token_actions::Burn {
            mint,
            account,
            owner,
            amount,
        })
        .into(),
    ))
}

fn classify_close_account(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let destination = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::CloseAccount(token_actions::CloseAccount {
            account,
            destination,
            owner,
        })
        .into(),
    ))
}

fn classify_freeze_account(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let freeze_authority = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::FreezeAccount(token_actions::FreezeAccount {
            account,
            mint,
            freeze_authority,
        })
        .into(),
    ))
}

fn classify_thaw_account(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let freeze_authority = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::ThawAccount(token_actions::ThawAccount {
            account,
            mint,
            freeze_authority,
        })
        .into(),
    ))
}

fn classify_transfer_checked(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
    decimals: u8,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 4)?;

    let source = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let destination = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let owner = txn
        .get_pubkey(ix.accounts[3])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::TransferChecked(token_actions::TransferChecked {
            source,
            mint,
            destination,
            owner,
            amount,
            decimals,
        })
        .into(),
    ))
}

fn classify_approve_checked(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
    decimals: u8,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 4)?;

    let source = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let delegate = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let owner = txn
        .get_pubkey(ix.accounts[3])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::ApproveChecked(token_actions::ApproveChecked {
            source,
            mint,
            delegate,
            owner,
            amount,
            decimals,
        })
        .into(),
    ))
}

fn classify_mint_to_checked(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
    decimals: u8,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let account = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint_authority = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::MintToChecked(token_actions::MintToChecked {
            mint,
            account,
            mint_authority,
            amount,
            decimals,
        })
        .into(),
    ))
}

fn classify_burn_checked(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
    decimals: u8,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    let owner = txn
        .get_pubkey(ix.accounts[2])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::BurnChecked(token_actions::BurnChecked {
            mint,
            account,
            owner,
            amount,
            decimals,
        })
        .into(),
    ))
}

fn classify_initialize_account2(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    owner: Pubkey,
) -> ClassifyInstructionResult {
    check_account_len(ix, 3)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeAccount(token_actions::InitializeAccount {
            account,
            mint,
            owner,
        })
        .into(),
    ))
}

fn classify_sync_native(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::SyncNative(token_actions::SyncNative { account }).into(),
    ))
}

fn classify_initialize_account3(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    owner: Pubkey,
) -> ClassifyInstructionResult {
    check_account_len(ix, 2)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;
    let mint = txn
        .get_pubkey(ix.accounts[1])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeAccount(token_actions::InitializeAccount {
            account,
            mint,
            owner,
        })
        .into(),
    ))
}

fn classify_initialize_multisig2(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len_gte(ix, 1)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeMultisig2(token_actions::InitializeMultisig2 { account }).into(),
    ))
}

fn classify_initialize_mint2(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    decimals: u8,
    mint_authority: Pubkey,
    freeze_authority: Option<Pubkey>,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeMint2(token_actions::InitializeMint2 {
            mint,
            decimals,
            mint_authority,
            freeze_authority,
        })
        .into(),
    ))
}

fn classify_get_account_data_size(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::GetAccountDataSize(token_actions::GetAccountDataSize { mint }).into(),
    ))
}

fn classify_initialize_immutable_owner(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let account = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::InitializeImmutableOwner(token_actions::InitializeImmutableOwner { account }).into(),
    ))
}

fn classify_amount_to_ui_amount(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    amount: u64,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::AmountToUiAmount(token_actions::AmountToUiAmount { mint, amount }).into(),
    ))
}

fn classify_ui_amount_to_amount(
    txn: &ClassifiableTransaction,
    ix: &ClassifiableInstruction,
    ui_amount: &str,
) -> ClassifyInstructionResult {
    check_account_len(ix, 1)?;

    let mint = txn
        .get_pubkey(ix.accounts[0])
        .ok_or_else(|| anyhow::anyhow!("Failed to get account pubkey from transaction"))?;

    Ok(Some(
        Token::UiAmountToAmount(token_actions::UiAmountToAmount {
            mint,
            ui_amount: ui_amount.to_string(),
        })
        .into(),
    ))
}
