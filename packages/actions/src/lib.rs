mod block;
mod jito;
mod post_processing;
mod protocols;
mod solana;
mod transaction;

pub use block::*;
pub use jito::*;
pub use post_processing::*;
pub use protocols::*;
pub use solana::*;
pub use transaction::*;

use classifier_core::ClassifiableTransaction;
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
    ClassifiableTransaction,
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
    WhirlpoolsAction,
    JupiterV6Action,
    MeteoraDlmmAction,
    RaydiumClmmAction,
    // Post processing actions
    JitoBundle,
    DexSwap,
}
