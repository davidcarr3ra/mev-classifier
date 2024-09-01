mod block;
mod dex_swap;
mod jito;
mod solana;
mod transaction;

pub use block::*;
pub use dex_swap::*;
pub use jito::*;
pub use solana::*;
pub use transaction::*;

use macros::define_actions;

define_actions! {
    // Name of generated struct
    Action,

    // Trait all actions must implement
    pub trait ActionTrait {
        /// Helper function for the instruction classifier. If an action stems from both
        /// 1) An instruction which may have inner instructions, and
        /// 2) Inner instructions which may contain more useful actions,
        ///
        /// then this should return true. For actions which are known to produce no
        /// more useful info regardless of inner instructions, return false.
        fn recurse_during_classify(&self) -> bool;
    },

    //
    // Registered variants (must implement above trait and #[action] or #[action_enum])
    //

    // Utility
    Transaction,
    Block,

    // Solana system actions
    ProgramInvocation,
    NativeTransfer,
    Vote,
    SetComputeBudgetLimit,
    SetComputeUnitPrice,

    // 3rd party actions
    JitoTip,
    JitoBundle,
    DexSwap,
}
