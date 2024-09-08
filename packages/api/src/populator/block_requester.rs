use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use crossbeam::channel;
use inspection::database::{document_builder::BlockDocuments, mongo_client::MongoDBClient};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, Semaphore, TryAcquireError};

use crate::populator::ClassifyBlockRequest;

use super::ClassifyResult;

pub type IndexBlockRequest = Vec<(u64, oneshot::Sender<bool>)>;

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
        request_rx: mpsc::Receiver<IndexBlockRequest>,
        classifier_tx: crossbeam::channel::Sender<ClassifyBlockRequest>,
        classify_result_rx: mpsc::Receiver<ClassifyResult>,
        rpc_client: Arc<RpcClient>,
        mongo_client: MongoDBClient,
        config: BlockRequesterConfig,
    ) -> Self {
        let (alert_tx, alert_rx) = mpsc::channel(10_000);

        let mut thread = BlockRequesterThread {
            requests_per_period: config.requests_per_period,
            period: config.period,
            receiver: request_rx,
            classifier_tx,
            classify_result_rx,
            alert_handle: AlertHandle::new(alert_tx),
            alert_rx,
            rpc_client,
            mongo_client,
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

#[derive(Clone)]
struct AlertHandle {
    alert_tx: mpsc::Sender<(u64, bool)>,
}

impl AlertHandle {
    fn new(alert_tx: mpsc::Sender<(u64, bool)>) -> Self {
        Self { alert_tx }
    }

    async fn alert(&self, slot: u64, result: bool) {
        match self.alert_tx.send((slot, result)).await {
            Ok(_) => {}
            Err(_) => {
                tracing::error!("Failed to send alert");
            }
        }
    }
}

pub(crate) struct BlockRequesterThread {
    pub requests_per_period: usize,
    pub period: Duration,

    receiver: mpsc::Receiver<IndexBlockRequest>,
    classifier_tx: crossbeam::channel::Sender<ClassifyBlockRequest>,
    classify_result_rx: mpsc::Receiver<ClassifyResult>,

    // Channels for scheduling alerts
    alert_handle: AlertHandle,
    alert_rx: mpsc::Receiver<(u64, bool)>,

    rpc_client: Arc<RpcClient>,
    mongo_client: MongoDBClient,
    semaphore: Semaphore,
    in_progress: HashMap<u64, Vec<oneshot::Sender<bool>>>,
    pending_queue: VecDeque<u64>,
}

impl BlockRequesterThread {
    pub(crate) async fn thread_loop(&mut self) {
        // let mut semaphore = Semaphore::new(self.requests_per_period);
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
                recv = self.receiver.recv() => {
                    let requests = match recv {
                        Some(v) => v,
                        None => break,
                    };
                    tracing::trace!("Received {} slot requests", requests.len());

                    // Add slots to in progress set
                    for (slot, response) in requests {
                        if self.in_progress.contains_key(&slot) {
                            continue;
                        }

                        self.pending_queue.push_back(slot);
                        self.in_progress.entry(slot).or_default().push(response);
                    }

                    // Try to request blocks
                    while self.semaphore.available_permits() > 0 && !self.pending_queue.is_empty() {
                        self.try_request_block();
                    }
                }

                // Receive classifications and store in DB
                classify_result = self.classify_result_rx.recv() => {
                    let classify_result = match classify_result {
                        Some(v) => v,
                        None => break,
                    };

                    let (slot, result) = classify_result;

                    match result {
                        Ok(classification) => {
                            self.commit_classifications(slot, classification, self.alert_handle.clone()).await;
                        }
                        Err(err) => {
                           tracing::error!("Failed to classify block: {:?}", err);
                            self.alert_result(slot, false);
                        }
                    };
                }

                // Receive alerts from other threads
                alert = self.alert_rx.recv() => {
                    let alert = match alert {
                        Some(v) => v,
                        None => break,
                    };

                    let (slot, result) = alert;
                    self.alert_result(slot, result);
                }
            }
        }

        tracing::debug!("BlockRequesterThread exiting");
    }

    fn alert_result(&mut self, slot: u64, result: bool) {
        let responses = self.in_progress.remove(&slot).unwrap_or_default();
        for response in responses {
            match response.send(result) {
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
        classifier_tx: channel::Sender<ClassifyBlockRequest>,
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

    async fn commit_classifications(
        &self,
        slot: u64,
        block_documents: BlockDocuments,
        alert_handle: AlertHandle,
    ) {
        let mongo_client = self.mongo_client.clone();
        tokio::spawn(async move {
            tracing::trace!("Writing classification to MongoDB: slot {}", slot);
            let result = mongo_client.write_block_documents(block_documents).await;
            if let Err(err) = &result {
                tracing::error!("Failed to write classification to MongoDB: {:?}", err);
            }

            tracing::trace!("Wrote classification to MongoDB: slot {}", slot);

            alert_handle.alert(slot, result.is_ok()).await;
        });
    }
}
