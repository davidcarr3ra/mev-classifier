mod block;
mod jito;
mod post_processing;
mod protocols;
mod serialize_tree;
mod solana;
mod transaction;

pub use block::*;
pub use jito::*;
pub use post_processing::*;
pub use protocols::*;
pub use serialize_tree::*;
pub use solana::*;

use classifier_core::ClassifiableTransaction;
use macros::define_actions;

pub type ActionTree = action_tree::ActionTree<Action>;
pub type ActionNodeId = action_tree::ActionNodeId;
pub type ActionNode = action_tree::ActionNode<Action>;
pub type ActionNodeEdge = action_tree::ActionNodeEdge;
pub type ActionDescendants<'a> = action_tree::ActionDescendants<'a, Action>;

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

        /// Optionally convert action into DexSwap during post processing
        #[allow(unused_variables)]
        fn into_dex_swap(
            &self,
            txn: &ClassifiableTransaction,
            action_id: ActionNodeId,
            tree: &ActionTree
        ) -> Result<Option<DexSwap>, anyhow::Error> {
            Ok(None)
        }

        fn serializable(&self) -> bool {
            false
        }

        fn to_json(&self) -> serde_json::Value {
            unreachable!("Action should not be serialized")
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
    Token,
    AssociatedToken,

    // 3rd party actions
    JitoTip,
    BloxrouteTip,
    WhirlpoolsAction,
    JupiterV6Action,
    MeteoraDlmmAction,
    RaydiumClmmAction,
    RaydiumAmmAction,
    PhoenixV1Action,
    StarAtlasAction,

    // Post processing actions (labels)
    JitoBundle,
    DexSwap,
}
