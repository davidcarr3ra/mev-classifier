mod jupiter_v6;
mod whirlpools;

pub use jupiter_v6::*;
use macros::action_enum;
pub use whirlpools::*;

use super::ActionTrait;

#[action_enum]
pub enum DexSwap {
    Whirlpools(WhirlpoolsSwapAction),
    JupiterV6(JupiterV6SwapAction),
    JupiterV6Ledger(JupiterV6LedgerSwapAction),
}

impl DexSwap {
    pub(crate) fn recurse_during_classify(&self) -> bool {
        match self {
            DexSwap::Whirlpools(_) => false,
            DexSwap::JupiterV6(_) => true,
            DexSwap::JupiterV6Ledger(_) => true,
        }
    }
}

impl ActionTrait for DexSwap {
    fn recurse_during_classify(&self) -> bool {
        self.recurse_during_classify()
    }
}
