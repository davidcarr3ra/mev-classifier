use macros::action_enum;

use crate::ActionTrait;

#[action_enum]
pub enum Token {
    InitializeMint(crate::token_actions::InitializeMint),
    InitializeAccount(crate::token_actions::InitializeAccount),
    InitializeMultisig(crate::token_actions::InitializeMultisig),
    Transfer(crate::token_actions::Transfer),
    Approve(crate::token_actions::Approve),
    Revoke(crate::token_actions::Revoke),
    SetAuthority(crate::token_actions::SetAuthority),
    MintTo(crate::token_actions::MintTo),
    Burn(crate::token_actions::Burn),
    CloseAccount(crate::token_actions::CloseAccount),
    FreezeAccount(crate::token_actions::FreezeAccount),
    ThawAccount(crate::token_actions::ThawAccount),
    TransferChecked(crate::token_actions::TransferChecked),
    ApproveChecked(crate::token_actions::ApproveChecked),
    MintToChecked(crate::token_actions::MintToChecked),
    BurnChecked(crate::token_actions::BurnChecked),
    InitializeAccount2(crate::token_actions::InitializeAccount2),
    SyncNative(crate::token_actions::SyncNative),
    InitializeAccount3(crate::token_actions::InitializeAccount3),
    InitializeMultisig2(crate::token_actions::InitializeMultisig2),
    InitializeMint2(crate::token_actions::InitializeMint2),
    GetAccountDataSize(crate::token_actions::GetAccountDataSize),
    InitializeImmutableOwner(crate::token_actions::InitializeImmutableOwner),
    AmountToUiAmount(crate::token_actions::AmountToUiAmount),
    UiAmountToAmount(crate::token_actions::UiAmountToAmount),
}

impl ActionTrait for Token {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}

pub mod token_actions {
    use macros::action;
    use solana_sdk::pubkey::Pubkey;

    #[action]
    pub struct InitializeMint {
        pub mint: Pubkey,
    }

    #[action]
    pub struct InitializeAccount {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub owner: Pubkey,
    }

    #[action]
    pub struct InitializeMultisig {
        pub account: Pubkey,
    }

    #[action]
    pub struct Transfer {
        pub source: Pubkey,
        pub destination: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
    }

    #[action]
    pub struct Approve {
        pub source: Pubkey,
        pub delegate: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
    }

    #[action]
    pub struct Revoke {
        pub source: Pubkey,
        pub owner: Pubkey,
    }

    #[action]
    pub struct SetAuthority {
        pub account: Pubkey,
        pub current_authority: Pubkey,
        pub authority_type: u8,
        pub new_authority: Option<Pubkey>,
    }

    #[action]
    pub struct MintTo {
        pub mint: Pubkey,
        pub account: Pubkey,
        pub mint_authority: Pubkey,
        pub amount: u64,
    }

    #[action]
    pub struct Burn {
        pub mint: Pubkey,
        pub account: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
    }

    #[action]
    pub struct CloseAccount {
        pub account: Pubkey,
        pub destination: Pubkey,
        pub owner: Pubkey,
    }

    #[action]
    pub struct FreezeAccount {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub freeze_authority: Pubkey,
    }

    #[action]
    pub struct ThawAccount {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub freeze_authority: Pubkey,
    }

    #[action]
    pub struct TransferChecked {
        pub source: Pubkey,
        pub mint: Pubkey,
        pub destination: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
        pub decimals: u8,
    }

    #[action]
    pub struct ApproveChecked {
        pub source: Pubkey,
        pub mint: Pubkey,
        pub delegate: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
        pub decimals: u8,
    }

    #[action]
    pub struct MintToChecked {
        pub mint: Pubkey,
        pub account: Pubkey,
        pub mint_authority: Pubkey,
        pub amount: u64,
        pub decimals: u8,
    }

    #[action]
    pub struct BurnChecked {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub owner: Pubkey,
        pub amount: u64,
        pub decimals: u8,
    }

    #[action]
    pub struct InitializeAccount2 {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub owner: Pubkey,
    }

    #[action]
    pub struct SyncNative {
        pub account: Pubkey,
    }

    #[action]
    pub struct InitializeAccount3 {
        pub account: Pubkey,
        pub mint: Pubkey,
        pub owner: Pubkey,
    }

    #[action]
    pub struct InitializeMultisig2 {
        pub account: Pubkey,
    }

    #[action]
    pub struct InitializeMint2 {
        pub mint: Pubkey,
        pub decimals: u8,
        pub mint_authority: Pubkey,
        pub freeze_authority: Option<Pubkey>,
    }

    #[action]
    pub struct GetAccountDataSize {
        pub mint: Pubkey,
    }

    #[action]
    pub struct InitializeImmutableOwner {
        pub account: Pubkey,
    }

    #[action]
    pub struct AmountToUiAmount {
        pub mint: Pubkey,
        pub amount: u64,
    }

    #[action]
    pub struct UiAmountToAmount {
        pub mint: Pubkey,
        pub ui_amount: String,
    }
}
