mod classify;

pub use classify::*;

use crate::populator::FetchBlockSender;

#[derive(Clone)]
pub struct AppState {
    pub user_request_tx: FetchBlockSender,
}
