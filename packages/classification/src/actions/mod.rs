mod block;
mod dex_swap;
mod jito_tip;
mod native_transfer;
mod transaction;

pub use block::*;
pub use dex_swap::*;
pub use jito_tip::*;
pub use native_transfer::*;
pub use transaction::*;

#[derive(Debug)]
pub enum Action {
    NativeTransfer(NativeTransferAction),
    JitoTip(JitoTipAction),
    Transaction(TransactionAction),
    Block(BlockAction),
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

            Action::Transaction(_) => unreachable!("Instructions can not be transactions"),
            Action::Block(_) => unreachable!("Instructions can not be blocks"),
        }
    }
}
