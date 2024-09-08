mod classify;

pub use classify::*;

use crate::populator::IndexBlockRequest;

#[derive(Clone)]
pub struct AppState {
    pub request_tx: tokio::sync::mpsc::Sender<IndexBlockRequest>,
}
