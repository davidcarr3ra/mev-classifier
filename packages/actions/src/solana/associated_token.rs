use macros::action_enum;

use crate::ActionTrait;

#[action_enum]
pub enum AssociatedToken {
    Create(crate::associated_token_actions::Create),
    CreateIdempotent(crate::associated_token_actions::CreateIdempotent),
    RecoverNested,
}

impl ActionTrait for AssociatedToken {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}

pub mod associated_token_actions {
    use macros::action;
    use solana_sdk::pubkey::Pubkey;

    #[action]
    pub struct Create {
        pub payer: Pubkey,
        pub associated_token_address: Pubkey,
        pub wallet: Pubkey,
        pub mint: Pubkey,
        pub token_program: Pubkey,
    }

    #[action]
    pub struct CreateIdempotent {
        pub payer: Pubkey,
        pub associated_token_address: Pubkey,
        pub wallet: Pubkey,
        pub mint: Pubkey,
        pub token_program: Pubkey,
    }
}
