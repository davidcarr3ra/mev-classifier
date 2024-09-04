use macros::declare_anchor_actions;

use crate::ActionTrait;

declare_anchor_actions!(
    raydium_clmm,
    Swap {
        Args: {
            amount,
            is_base_input,
        },
        Accounts: {
            pool_state,
            input_token_account,
            output_token_account,
        }
    }
);

impl ActionTrait for RaydiumClmmAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
