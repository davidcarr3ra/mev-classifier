use macros::declare_anchor_actions;

use crate::ActionTrait;

declare_anchor_actions!(
    jupiter_v6,
    Route {
        Args: {
            in_amount,
            quoted_out_amount,
            slippage_bps,
        },
        Accounts: {
            user_source_token_account,
            user_destination_token_account,
        },
    },
);

impl ActionTrait for JupiterV6Action {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}