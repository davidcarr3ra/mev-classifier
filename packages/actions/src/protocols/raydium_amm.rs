use macros::action_enum;

use crate::ActionTrait;

#[action_enum]
pub enum RaydiumAmmAction {
    Initialize,
    Initialize2,
    Preinitialize,
    MonitorStep,
    Deposit,
    Withdraw,
    MigrateToOpenBook,
    SetParams,
    WithdrawPnl,
    WithdrawSrm,
    SwapBaseIn,
    SwapBaseOut,
    SimulateInstruction,
    AdminCancelOrders,
    CreateConfigAccount,
    UpdateConfigAccount,
}

impl ActionTrait for RaydiumAmmAction {
    fn recurse_during_classify(&self) -> bool {
        false
    }
}
