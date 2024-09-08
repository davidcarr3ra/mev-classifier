use std::sync::Arc;

use axum::{extract::Query, Extension};
use serde::Deserialize;

use super::AppState;

#[derive(Deserialize)]
pub struct ClassifyQuery {
    pub start_slot: u64,
    pub limit: u64,
}

pub async fn classify(
    Query(params): Query<ClassifyQuery>,
    Extension(state): Extension<Arc<AppState>>,
) -> &'static str {
    let ClassifyQuery { start_slot, limit } = params;

    let mut requests = Vec::with_capacity(limit as usize);
    let mut recievers = Vec::with_capacity(limit as usize);
    for slot in start_slot..start_slot + limit {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let request = (slot, tx);
        requests.push(request);
        recievers.push(rx);
    }

    if state.request_tx.send(requests).await.is_err() {
        return "error";
    }

    // Wait for all requests to be processed
    // TODO: Timeout
    futures::future::join_all(recievers).await;

    "success"
}
