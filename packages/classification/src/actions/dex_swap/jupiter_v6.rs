use macros::action;

use super::DexSwap;

#[action]
pub struct JupiterV6SwapAction {
    pub in_amount: u64,
    pub quoted_out_amount: u64,
}

impl Into<DexSwap> for JupiterV6SwapAction {
    fn into(self) -> DexSwap {
        DexSwap::JupiterV6(self)
    }
}

/// Token ledger swaps do not reveal in amount
#[action]
pub struct JupiterV6LedgerSwapAction {
    pub quoted_out_amount: u64,
}

impl Into<DexSwap> for JupiterV6LedgerSwapAction {
    fn into(self) -> DexSwap {
        DexSwap::JupiterV6Ledger(self)
    }
}
