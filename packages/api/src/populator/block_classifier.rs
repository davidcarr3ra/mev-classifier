use std::thread;

use classifier_handler::classify_block;
use inspection::{
    database::document_builder::{self, BlockDocuments},
    filtering::{post_process, PostProcessConfig},
    label_tree,
};
use solana_transaction_status::UiConfirmedBlock;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum ClassifyBlockError {
    #[error("Failed to classify block: {0}")]
    ClassifyError(#[from] classifier_handler::ClassifyBlockError),

    #[error("Failed to build block documents: {0}")]
    BuildError(#[from] document_builder::DocumentBuilderError),
}

pub struct ClassifyBlockRequest {
    pub slot: u64,
    pub block: UiConfirmedBlock,
}

pub type ClassifyResult = (u64, Result<BlockDocuments, ClassifyBlockError>);

pub struct BlockClassifier {
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl BlockClassifier {
    pub fn new(
        request_rx: crossbeam::channel::Receiver<ClassifyBlockRequest>,
        result_tx: mpsc::Sender<ClassifyResult>,
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
    result_tx: mpsc::Sender<ClassifyResult>,
}

impl BlockClassifierThread {
    pub(crate) fn thread_loop(&mut self) {
        while let Ok(request) = self.request_rx.recv() {
            let classify_result = classify_block(request.slot, request.block, None)
                .map_err(ClassifyBlockError::ClassifyError);

            // Handle classification error
            if let Err(err) = classify_result {
                match self
                    .result_tx
                    .blocking_send((request.slot, Err(err)))
                {
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

            let doc_result = document_builder::build_block_documents(&tree, tree.root())
                .map_err(ClassifyBlockError::BuildError);

            // Send docs
            match self.result_tx.blocking_send((request.slot, doc_result)) {
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
