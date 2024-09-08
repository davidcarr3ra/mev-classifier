use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing::get, Extension, Router};
use inspection::database::mongo_client::{
    MongoDBClient, MongoDBClientConfig, MongoDBClientError, MongoDBStage,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use thiserror::Error;

use crate::{
    populator::{BlockClassifier, BlockRequester, BlockRequesterConfig},
    routes::{classify, AppState},
};

pub enum TimeMachineStage {
    Beta,
}

impl Into<MongoDBStage> for TimeMachineStage {
    fn into(self) -> MongoDBStage {
        match self {
            TimeMachineStage::Beta => MongoDBStage::Beta,
        }
    }
}

#[derive(Debug, Error)]
pub enum TimeMachineError {
    #[error("Failed to create MongoDB client: {0}")]
    MongoDBError(#[from] MongoDBClientError),

    #[error("Failed to bind to address: {0}")]
    BindError(#[source] std::io::Error),

    #[error("Server error: {0}")]
    ServerError(#[source] std::io::Error),
}

type Result<T> = std::result::Result<T, TimeMachineError>;

pub struct TimeMachineServerConfig {
    pub rpc_url: String,
    pub port: u16,
    pub rpc_requests_per_second: usize,
    pub mongo_uri: String,
    pub stage: TimeMachineStage,
}

pub struct TimeMachineServer {
    addr: SocketAddr,
    rpc_client: Arc<RpcClient>,
    mongo_client: MongoDBClient,
    rpc_requests_per_second: usize,
}

impl TimeMachineServer {
    pub async fn new(config: TimeMachineServerConfig) -> Result<Self> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port);

        let rpc_client = Arc::new(RpcClient::new(config.rpc_url));
        let mongo_client = MongoDBClient::new(MongoDBClientConfig {
            stage: config.stage.into(),
            uri: config.mongo_uri,
        })
        .await?;

        Ok(Self {
            addr,
            rpc_client,
            mongo_client,
            rpc_requests_per_second: config.rpc_requests_per_second,
        })
    }

    pub async fn serve(&self) -> Result<()> {
        // Setup up processing pipeline (one thread coordinates async work, one thread classifies blocks)
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(10_000);
        let (populator_tx, populator_rx) = crossbeam::channel::unbounded();
        let (classify_result_tx, classify_result_rx) = tokio::sync::mpsc::channel(10_000);
        let block_requester = BlockRequester::new(
            request_rx,
            populator_tx,
            classify_result_rx,
            self.rpc_client.clone(),
            self.mongo_client.clone(),
            BlockRequesterConfig {
                requests_per_period: self.rpc_requests_per_second,
                period: std::time::Duration::from_secs(1),
            },
        );

        let block_populator = BlockClassifier::new(populator_rx, classify_result_tx);

        // Setup routes
        let classify_state = Arc::new(AppState { request_tx });
        let app = Router::new()
            .route("/classify", get(classify))
            .layer(Extension(classify_state));

        // Start server
        let tcp_listener = tokio::net::TcpListener::bind(&self.addr)
            .await
            .map_err(TimeMachineError::BindError)?;

        println!("Listening on {}", self.addr);
        tracing::info!("Listening on {}", self.addr);
        axum::serve(tcp_listener, app)
            .await
            .map_err(TimeMachineError::ServerError)?;

        // Cleanup
        drop(block_requester);
        drop(block_populator);

        Ok(())
    }
}
