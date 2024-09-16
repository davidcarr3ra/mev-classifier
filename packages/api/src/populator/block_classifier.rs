use std::thread;

use actions::ActionTree;
use classifier_handler::classify_block;
use inspection::{
    filtering::{post_process, PostProcessConfig},
    label_tree,
};
use solana_transaction_status::UiConfirmedBlock;
use tokio::sync::mpsc;

pub struct ClassifyBlockRequest {
    pub slot: u64,
    pub block: UiConfirmedBlock,
}
pub type ClassifyBlockRequestSender = crossbeam::channel::Sender<ClassifyBlockRequest>;

pub type ClassifyResult = Option<ActionTree>;
pub type ClassifyBlockResponse = (u64, ClassifyResult);
pub type ClassifyBlockResponseReceiver = mpsc::Receiver<ClassifyBlockResponse>;

pub struct BlockClassifier {
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl BlockClassifier {
    pub fn new(
        request_rx: crossbeam::channel::Receiver<ClassifyBlockRequest>,
        result_tx: mpsc::Sender<ClassifyBlockResponse>,
    ) -> Self {
        let mut thread = BlockClassifierThread {
            request_rx,
            result_tx,
        };

        let handle = thread::spawn(move || thread.thread_loop());

        Self {
            thread_handle: Some(handle),
        }
    }
}

impl Drop for BlockClassifier {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}

struct BlockClassifierThread {
    request_rx: crossbeam::channel::Receiver<ClassifyBlockRequest>,
    result_tx: mpsc::Sender<ClassifyBlockResponse>,
}

impl BlockClassifierThread {
    pub(crate) fn thread_loop(&mut self) {
        while let Ok(request) = self.request_rx.recv() {
            let classify_result = classify_block(request.slot, request.block, None);

            // Handle classification error
            if let Err(err) = classify_result {
                tracing::error!("Failed to classify block: {:?}", err);

                match self.result_tx.blocking_send((request.slot, None)) {
                    Ok(_) => {}
                    Err(_) => {
                        tracing::error!("Failed to send classify result");
                        break;
                    }
                }
                continue;
            }

            // Post process the result
            let mut tree = classify_result.unwrap();
            label_tree(&mut tree);

            post_process(
                PostProcessConfig {
                    retain_votes: false,
                    remove_empty_transactions: true,
                    cluster_jito_bundles: true,
                },
                &mut tree,
            );

            // Send docs
            match self.result_tx.blocking_send((request.slot, Some(tree))) {
                Ok(_) => {}
                Err(_) => {
                    tracing::error!("Failed to send classify result");
                    break;
                }
            }
        }

        tracing::debug!("BlockPopulatorThread exiting");
    }
}
