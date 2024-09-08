use std::time::Instant;

use mongodb::{
    bson::{doc, Document},
    error::UNKNOWN_TRANSACTION_COMMIT_RESULT,
    options::{ClientOptions, TransactionOptions, WriteConcern},
    Client, Collection,
};

use super::document_builder::BlockDocuments;

pub type MongoDBClientError = mongodb::error::Error;
type Result<T> = std::result::Result<T, MongoDBClientError>;

pub enum MongoDBStage {
    Beta,
}

impl ToString for MongoDBStage {
    fn to_string(&self) -> String {
        match self {
            MongoDBStage::Beta => "beta".to_string(),
        }
    }
}

pub struct MongoDBClientConfig {
    pub uri: String,
    pub stage: MongoDBStage,
}

#[derive(Clone)]
pub struct MongoDBClient {
    client: Client,
    database_name: String,
}

impl MongoDBClient {
    pub async fn new(config: MongoDBClientConfig) -> Result<Self> {
        let mut client_options = ClientOptions::parse(config.uri).await?;
        client_options.connect_timeout = Some(std::time::Duration::from_secs(10));

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(err) => {
                tracing::error!("Failed to create MongoDB client: {:?}", err);
                return Err(err);
            }
        };

        tracing::info!("Connected to MongoDB");

        Ok(Self {
            client,
            database_name: format!("time-machine-{}", config.stage.to_string()),
        })
    }

    pub async fn write_block_documents(&self, block_documents: BlockDocuments) -> Result<()> {
        let timestamp = Instant::now();
        tracing::trace!("Writing block documents to MongoDB");

        // Start a session for the transaction
        let mut session = self.client.start_session().await?;

        let txn_options = TransactionOptions::builder()
            .write_concern(WriteConcern::majority())
            .build();

        // Start the transaction
        session
            .start_transaction()
            .with_options(txn_options)
            .await?;

        // Reference the database and collections
        let db = self.client.database(&self.database_name);
        let blocks_collection: Collection<Document> = db.collection("blocks");

        let block_id = block_documents.block.get("_id").unwrap().clone();
        let block_filter = doc! { "_id": block_id.clone() };
        blocks_collection
            .replace_one(block_filter, block_documents.block)
            .upsert(true)
            .session(&mut session)
            .await?;

        // Delete and replace transactions
        if !block_documents.transactions.is_empty() {
            let transactions_collection = db.collection("transactions");

            transactions_collection
                .delete_many(doc! { "block_id": block_id.clone() })
                .session(&mut session)
                .await?;

            transactions_collection
                .insert_many(block_documents.transactions)
                .await?;
        }

        // Delete and replace metadata
        if !block_documents.block_metadata.is_empty() {
            let metadata_collection = db.collection("block_metadata");
            metadata_collection
                .delete_many(doc! { "block_id": block_id })
                .session(&mut session)
                .await?;

            metadata_collection
                .insert_many(block_documents.block_metadata)
                .await?;
        }

        // Commit transaction, retrying if necessary
        loop {
            let result = session.commit_transaction().await;
            if let Err(ref error) = result {
                tracing::error!("Error committing transaction: {:?}", error);

                if error.contains_label(UNKNOWN_TRANSACTION_COMMIT_RESULT) {
                    continue;
                }
            }

            match result {
                Ok(_) => {
                    tracing::trace!("Transaction committed in {:?}", timestamp.elapsed());
                    return Ok(());
                }
                Err(err) => {
                    tracing::error!("Failed to commit transaction: {:?}", err);
                    return Err(err);
                }
            }
        }
    }
}
