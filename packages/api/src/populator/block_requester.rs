use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use crossbeam::channel;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, Semaphore, TryAcquireError};

use crate::populator::ClassifyBlockRequest;

use super::{
    ClassifyBlockRequestSender, ClassifyBlockResponse, ClassifyBlockResponseReceiver,
    ClassifyResult,
};

pub type FetchBlockSender = mpsc::Sender<Vec<FetchBlockRequest>>;
pub type FetchBlockReceiver = mpsc::Receiver<Vec<FetchBlockRequest>>;

pub struct FetchBlockRequest {
    pub slot: u64,
    pub response: oneshot::Sender<ClassifyResult>,
}

#[derive(Debug, Error)]
pub enum BlockRequesterError {
    #[error("Classification request failed")]
    ClassifierDisconnected(#[from] channel::SendError<ClassifyBlockRequest>),

    #[error("RPC request failed: {0}")]
    RpcFailure(#[from] solana_client::client_error::ClientError),
}

type Result<T> = std::result::Result<T, BlockRequesterError>;

pub struct BlockRequesterConfig {
    pub requests_per_period: usize,
    pub period: Duration,
}

/// BlockRequester handles coordinating requests for block data from the RPC client
/// in order to respect RPC rate limits. It prioritizes user requests for block data
/// at all times, secondly prioritizing live block data indexing and lastly backfilling.
///
/// This thread is the entrypoint to the overall classification flow. It fetches blocks,
/// then forwards them to the classifier thread. Once the ActionTree has been created and labeled,
/// it will be sent to the onshot channel provided by the initial FetchBlockRequest.
pub struct BlockRequester {
    thread_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Drop for BlockRequester {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            tokio::task::block_in_place(|| handle.abort());
        }
    }
}

impl BlockRequester {
    pub fn new(
        user_request_rx: FetchBlockReceiver,
        classifier_tx: ClassifyBlockRequestSender,
        classify_result_rx: ClassifyBlockResponseReceiver,
        rpc_client: Arc<RpcClient>,
        config: BlockRequesterConfig,
    ) -> Self {
        let mut thread = BlockRequesterThread {
            requests_per_period: config.requests_per_period,
            period: config.period,
            user_request_rx,
            classifier_tx,
            classify_result_rx,
            rpc_client,
            semaphore: Semaphore::new(config.requests_per_period),
            in_progress: HashMap::new(),
            pending_queue: VecDeque::new(),
        };

        let handle = tokio::spawn(async move { thread.thread_loop().await });

        Self {
            thread_handle: Some(handle),
        }
    }
}

pub(crate) struct BlockRequesterThread {
    pub requests_per_period: usize,
    pub period: Duration,

    user_request_rx: FetchBlockReceiver,
    classifier_tx: ClassifyBlockRequestSender,
    classify_result_rx: ClassifyBlockResponseReceiver,

    rpc_client: Arc<RpcClient>,
    semaphore: Semaphore,
    in_progress: HashMap<u64, Vec<oneshot::Sender<ClassifyResult>>>,
    pending_queue: VecDeque<u64>,
}

impl BlockRequesterThread {
    pub(crate) async fn thread_loop(&mut self) {
        let mut ticker = tokio::time::interval(self.period);

        loop {
            tokio::select! {
                // Control rate limit for RPC requests
                _ = ticker.tick() => {
                    self.semaphore.add_permits(self.requests_per_period);

                    while self.semaphore.available_permits() > 0 && !self.pending_queue.is_empty() {
                        self.try_request_block();
                    }
                }

                // Receive requests from users
                recv = self.user_request_rx.recv() => {
                    let requests = match recv {
                        Some(v) => v,
                        None => break,
                    };

                    self.handle_requests(requests);
                }

                // Receive classifications and dispatch to subscribers
                classify_result = self.classify_result_rx.recv() => {
                    let classify_result = match classify_result {
                        Some(v) => v,
                        None => break,
                    };

                    self.handle_result(classify_result);
                }
            }
        }

        tracing::debug!("BlockRequesterThread exiting");
    }

    fn handle_requests(&mut self, requests: Vec<FetchBlockRequest>) {
        tracing::trace!("Received {} FetchBlockRequests", requests.len());

        // Add slots to in progress set
        for request in requests {
            if self.in_progress.contains_key(&request.slot) {
                continue;
            }

            self.pending_queue.push_back(request.slot);
            self.in_progress
                .entry(request.slot)
                .or_default()
                .push(request.response);
        }

        // Try to request blocks until rate limit is reached
        while self.semaphore.available_permits() > 0 && !self.pending_queue.is_empty() {
            self.try_request_block();
        }
    }

    fn handle_result(&mut self, result: ClassifyBlockResponse) {
        let (slot, result) = result;
        tracing::trace!("Classified block for slot {}", slot);

        let responses = self.in_progress.remove(&slot).unwrap_or_default();
        for response in responses {
            match response.send(result.clone()) {
                Ok(_) => {}
                Err(_) => {
                    tracing::error!("Failed to send classification result");
                }
            }
        }
    }

    fn try_request_block(&mut self) {
        let permit = match self.semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(TryAcquireError::Closed) => unreachable!(),
            Err(TryAcquireError::NoPermits) => return,
        };

        // Safety check, but shouldn't be possible in thread_loop
        if self.pending_queue.is_empty() {
            return;
        }

        let slot = self.pending_queue.pop_front().unwrap();
        let rpc_client = self.rpc_client.clone();

        // TODO: If this task fails, retry/requeue on RPC failure
        // and break on channel error
        tokio::task::spawn(Self::request_task(
            rpc_client,
            slot,
            self.classifier_tx.clone(),
        ));

        permit.forget();
    }

    async fn request_task(
        rpc_client: Arc<RpcClient>,
        slot: u64,
        classifier_tx: ClassifyBlockRequestSender,
    ) -> Result<()> {
        let block = rpc_client
            .get_block_with_config(
                slot,
                RpcBlockConfig {
                    max_supported_transaction_version: Some(0),
                    encoding: Some(UiTransactionEncoding::Base64),
                    transaction_details: Some(TransactionDetails::Full),
                    ..Default::default()
                },
            )
            .await?;

        tracing::trace!("Received block for slot {}", slot);

        let populate_request = ClassifyBlockRequest { slot, block };
        classifier_tx.send(populate_request)?;

        Ok(())
    }
}
