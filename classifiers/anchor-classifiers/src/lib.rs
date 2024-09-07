use anchor_lang::{prelude::*, Discriminator};
use macros::declare_anchor_classifier;

declare_anchor_classifier!(whirlpools, Swap, SwapV2);
declare_anchor_classifier!(jupiter_v6, Route, RouteWithTokenLedger, SharedAccountsRoute);
declare_anchor_classifier!(meteora_dlmm, Swap, SwapExactOut);
declare_anchor_classifier!(raydium_clmm, Swap);

declare_anchor_classifier!(
    phoenix_v1,
    Swap,
    SwapWithFreeFunds,
    PlaceLimitOrder,
    PlaceLimitOrderWithFreeFunds,
    ReduceOrder,
    ReduceOrderWithFreeFunds,
    CancelAllOrders,
    CancelAllOrdersWithFreeFunds,
    CancelUpTo,
    CancelUpToWithFreeFunds,
    CancelMultipleOrdersById,
    CancelMultipleOrdersByIdWithFreeFunds,
    WithdrawFunds,
    DepositFunds,
    RequestSeat,
    PlaceMultiplePostOnlyOrders,
    PlaceMultiplePostOnlyOrdersWithFreeFunds,
    InitializeMarket,
    ClaimAuthority,
    NameSuccessor,
    ChangeMarketStatus,
    ChangeSeatStatus,
    RequestSeatAuthorized,
    EvictSeat,
    ForceCancelOrders,
    CollectFees,
    ChangeFeeRecipient,
);
