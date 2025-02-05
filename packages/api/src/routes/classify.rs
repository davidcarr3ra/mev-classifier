use std::{sync::Arc, time::Duration};

use actions::serialize_block;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::future;
use serde::{Deserialize, Serialize};
use tokio::{sync::oneshot, time::timeout};

use inspection::database::document_builder;

use crate::populator::FetchBlockRequest;

use super::AppState;

#[derive(Deserialize)]
pub struct ClassifyQuery {
    #[serde(rename = "startSlot")]
    pub start_slot: u64,
    pub limit: u64,
}

#[derive(Serialize)]
struct ClassifySuccess {
    blocks: Vec<serde_json::Value>,
    failures: Vec<u64>,
    db_failures: Vec<u64>,
}

#[derive(Serialize)]
struct ClassifyError {
    message: String,
}

enum ClassifyResponse {
    Success(ClassifySuccess),
    Error(ClassifyError),
}

impl IntoResponse for ClassifyResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ClassifyResponse::Success(success) => (StatusCode::OK, Json(success)).into_response(),
            ClassifyResponse::Error(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
            }
        }
    }
}

pub async fn classify(
    Query(params): Query<ClassifyQuery>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let ClassifyQuery { start_slot, limit } = params;

    let mut requests = Vec::with_capacity(limit as usize);
    let mut receivers = Vec::with_capacity(limit as usize);

    // Create requests and receivers for each slot
    for slot in start_slot..start_slot + limit {
        let (tx, rx) = oneshot::channel();
        requests.push(FetchBlockRequest { slot, response: tx });
        receivers.push(rx);
    }

    // Send requests to the central processing thread
    if state.user_request_tx.send(requests).await.is_err() {
        // 500 Internal Server Error
        return ClassifyResponse::Error(ClassifyError {
            message: "Internal server error".to_string(),
        });
    }

    let timeout_duration = Duration::from_secs(30);

    // Collect results from the receivers, applying a timeout
    // Prepare vectors to hold successes and failures
    let mut blocks = Vec::with_capacity(limit as usize);
    let mut failures = Vec::with_capacity(limit as usize);
    let mut db_failures = Vec::with_capacity(limit as usize);

    // Prepare vectors to hold Block Documents
    let mut block_documents_collection = Vec::with_capacity(limit as usize);

    // Process the receivers and classify each slot as success or failure
    let _ = future::join_all(receivers.into_iter().enumerate().map(|(i, receiver)| {
        let slot = start_slot + i as u64;
        async move {
            match timeout(timeout_duration, receiver).await {
                Ok(Ok(tree)) => Some((slot, tree)), // success
                _ => Some((slot, None)),            // failure
            }
        }
    }))
    .await
    .into_iter()
    .filter_map(|result| result)
    .for_each(|(slot, tree)| {
        if let Some(tree) = tree {
            let block_json = serialize_block(&tree, tree.root());
            blocks.push(block_json);

            let _ = match document_builder::build_block_documents(&tree, tree.root()) {
                Ok(doc) => {
                    block_documents_collection.push(doc);
                },
                Err(_) => {
                    db_failures.push(slot);
                }
            };
            
        } else {
            failures.push(slot);
        }
    });
    
    if let Err(e) = state.mongo_client.write_batch_block_documents(block_documents_collection).await {
        return ClassifyResponse::Error(ClassifyError {
            message: format!("Failed to write block documents: {}", e)
        });
    }
    
    
    // Return the results as JSON
    ClassifyResponse::Success(ClassifySuccess { blocks, failures, db_failures})

}
