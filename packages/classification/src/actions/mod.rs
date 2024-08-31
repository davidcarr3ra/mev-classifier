mod block;
mod dex_swap;
mod jito_tip;
mod native_transfer;
mod program;
mod transaction;
mod vote;
mod jito_bundles;

pub use block::*;
pub use dex_swap::*;
pub use jito_tip::*;
pub use native_transfer::*;
pub use program::*;
pub use transaction::*;
pub use vote::*;
pub use jito_bundles::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Transaction(TransactionAction),
    Block(BlockAction),
    ProgramInvocation(ProgramInvocationAction),
    Vote(VoteAction),
    NativeTransfer(NativeTransferAction),
    JitoTip(JitoTipAction),
    DexSwap(DexSwapAction),
    JitoBundle(JitoBundleAction),
}

impl Action {
    /// Helper function for the instruction classifier. If an action stems from both
    /// 1) An instruction which may have inner instructions, and
    /// 2) Inner instructions which may contain more useful actions,
    ///
    /// then this should return true. For actions which are known to produce no
    /// more useful info regardless of inner instructions, return false.
    pub(crate) fn recurse_during_classify(&self) -> bool {
        match self {
            Action::NativeTransfer(_) => false,
            Action::JitoTip(_) => false,
            Action::DexSwap(dex_swap) => dex_swap.recurse_during_classify(),
            Action::ProgramInvocation(_) => true,
            Action::Vote(vote) => vote.recurse_during_classify(),
            Action::JitoBundle(_) => false,
            Action::Transaction(_) => unreachable!("Instructions can not be transactions"),
            Action::Block(_) => unreachable!("Instructions can not be blocks"),
            
        }
    }
}
