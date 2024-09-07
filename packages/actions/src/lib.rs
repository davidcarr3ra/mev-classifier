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
        fn recurse_during_classify(&self) -> bool {
            true
        }

        /// In DB, all children are grouped under a parent which is a child of a block node.
        /// This will typically be a Transaction node, but may be other nodes based on its
        /// classification. This function determines whether this action should be a
        /// root level node, and thus where the document is constructed from.
        fn is_document_root(&self) -> bool {
            false
        }
    },

    //
    // Registered variants (must implement above trait and #[action] or #[action_enum])
    //

    // Utility
    ClassifiableTransaction,
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
    PhoenixV1Action,
    StarAtlasAction,

    // Post processing actions
    JitoBundle,
    DexSwap,
    AtomicArbitrage,
}
