use crate::Action;

use super::DexSwapAction;

#[derive(Debug, PartialEq, Eq)]
pub struct JupiterV6SwapAction {
    pub in_amount: u64,
    pub quoted_out_amount: u64,
}

impl Into<Action> for JupiterV6SwapAction {
    fn into(self) -> Action {
        Action::DexSwap(self.into())
    }
}

impl Into<DexSwapAction> for JupiterV6SwapAction {
    fn into(self) -> DexSwapAction {
        DexSwapAction::JupiterV6(self)
    }
}

/// Token ledger swaps do not reveal in amount
#[derive(Debug, PartialEq, Eq)]
pub struct JupiterV6LedgerSwapAction {
    pub quoted_out_amount: u64,
}

impl Into<Action> for JupiterV6LedgerSwapAction {
    fn into(self) -> Action {
        Action::DexSwap(self.into())
    }
}

impl Into<DexSwapAction> for JupiterV6LedgerSwapAction {
    fn into(self) -> DexSwapAction {
        DexSwapAction::JupiterV6Ledger(self)
    }
}
