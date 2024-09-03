use macros::declare_anchor_actions;

use crate::ActionTrait;

declare_anchor_actions!(
    meteora_dlmm, 
    Swap {
        Args: {
            amount_in,
        },
        Accounts: {
            lb_pair,
            user_token_in,
            user_token_out,
        },
    },
    SwapExactOut {
        Args: {
            out_amount,
        },
        Accounts: {
            lb_pair,
            user_token_in,
            user_token_out,
        }
    }
);

impl ActionTrait for MeteoraDlmmAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
