mod classify;

pub use classify::*;

use crate::populator::FetchBlockSender;

use inspection::database::mongo_client::MongoDBClient;

#[derive(Clone)]
pub struct AppState {
    pub user_request_tx: FetchBlockSender,
    pub mongo_client: MongoDBClient,
}
