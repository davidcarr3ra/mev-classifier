use macros::declare_anchor_actions;

use crate::ActionTrait;

declare_anchor_actions!(
    phoenix_v1,
    Swap {
        Args: {},
        Accounts: {},
    },
    SwapWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    PlaceLimitOrder {
        Args: {},
        Accounts: {},
    },
    PlaceLimitOrderWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    ReduceOrder {
        Args: {},
        Accounts: {},
    },
    ReduceOrderWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    CancelAllOrders {
        Args: {},
        Accounts: {},
    },
    CancelAllOrdersWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    CancelUpTo {
        Args: {},
        Accounts: {},
    },
    CancelUpToWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    CancelMultipleOrdersById {
        Args: {},
        Accounts: {},
    },
    CancelMultipleOrdersByIdWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    WithdrawFunds {
        Args: {},
        Accounts: {},
    },
    DepositFunds {
        Args: {},
        Accounts: {},
    },
    RequestSeat {
        Args: {},
        Accounts: {},
    },
    PlaceMultiplePostOnlyOrders {
        Args: {},
        Accounts: {},
    },
    PlaceMultiplePostOnlyOrdersWithFreeFunds {
        Args: {},
        Accounts: {},
    },
    InitializeMarket {
        Args: {},
        Accounts: {},
    },
    ClaimAuthority {
        Args: {},
        Accounts: {},
    },
    NameSuccessor {
        Args: {},
        Accounts: {},
    },
    ChangeMarketStatus {
        Args: {},
        Accounts: {},
    },
    ChangeSeatStatus {
        Args: {},
        Accounts: {},
    },
    RequestSeatAuthorized {
        Args: {},
        Accounts: {},
    },
    EvictSeat {
        Args: {},
        Accounts: {},
    },
    ForceCancelOrders {
        Args: {},
        Accounts: {},
    },
    CollectFees {
        Args: {},
        Accounts: {},
    },
    ChangeFeeRecipient {
        Args: {},
        Accounts: {},
    },
);

impl ActionTrait for PhoenixV1Action {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
