mod jupiter_v6;
mod whirlpools;

pub use jupiter_v6::*;
pub use whirlpools::*;

#[derive(Debug)]
pub enum DexSwapAction {
    Whirlpools(WhirlpoolsSwapAction),
    JupiterV6(JupiterV6SwapAction),
    JupiterV6Ledger(JupiterV6LedgerSwapAction),
}

impl DexSwapAction {
    pub(crate) fn recurse_during_classify(&self) -> bool {
        match self {
            DexSwapAction::Whirlpools(_) => false,
            DexSwapAction::JupiterV6(_) => true,
            DexSwapAction::JupiterV6Ledger(_) => true,
        }
    }
}
