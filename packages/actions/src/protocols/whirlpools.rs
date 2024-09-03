use macros::declare_anchor_actions;

use crate::ActionTrait;

declare_anchor_actions!(
    whirlpools,
    Swap {
        Args: {
            amount,
        },
        Accounts: {
            whirlpool,
            token_owner_account_a,
            token_owner_account_b,
        },
    },
    SwapV2 {},
);

impl ActionTrait for WhirlpoolsAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}