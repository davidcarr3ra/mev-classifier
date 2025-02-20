use crate::ActionTrait;
use borsh::BorshDeserialize;
use macros::action_enum;

#[derive(BorshDeserialize, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

#[derive(BorshDeserialize, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SelfTradeBehavior {
    Abort = 0,
    CancelProvide = 1,
    DecrementTake = 2,
}

#[action_enum]
pub enum PhoenixV1Action {
    Swap(SwapAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwapAction {
    ImmediateOrCancel {
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
        use_only_deposited_funds: bool,
    },
    Limit {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
    },
    PostOnly {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
    },
}

impl ActionTrait for PhoenixV1Action {
    fn recurse_during_classify(&self) -> bool {
        true
    }
}
