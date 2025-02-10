use clap::Args;
use classifier_handler::classify_block;
use inspection::database::mongo_client::{MongoDBClient, MongoDBClientConfig, MongoDBStage};
use inspection::filtering::{post_process, PostProcessConfig};
use inspection::{database, label_tree};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use retry::{retry, delay::Exponential};
use futures::stream::{self, StreamExt};
use tokio::task;
use std::sync::Arc;


#[derive(Args, Debug, Clone)]
pub struct InspectArgs {
    #[clap(long, help = "The slot to inspect. Cannot be used with --epoch")]
    slot: Option<u64>,

    #[clap(long, help = "The epoch to inspect. Cannot be used with --slot")]
    epoch: Option<u64>,

    #[clap(long, help = "Filter transactions by signature.")]
    filter_transaction: Option<String>,

    #[clap(long, help = "Write tree to results folder", default_value = "false")]
    write_tree: bool,

    #[clap(
        long,
        help = "RPC URL to use for fetching data.",
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    rpc_url: String,

    #[clap(long, help = "MongoDB URI to use for writing data.")]
    mongo_uri: Option<String>,
}

struct Inspector {
    rpc_client: RpcClient,
    args: InspectArgs,
}

impl Inspector {
	fn new(args: InspectArgs) -> Self {
			let rpc_client = RpcClient::new(args.rpc_url.clone());
			Self { rpc_client, args }
	}

	fn process_slot_with_retry(&self, slot: u64) {
		let block = match retryable_request(|| {
			self.rpc_client.get_block_with_config(
				slot,
				RpcBlockConfig {
						max_supported_transaction_version: Some(0),
						encoding: Some(UiTransactionEncoding::Base64),
						transaction_details: Some(TransactionDetails::Full),
						..Default::default()
				},
			)
		}) {
			Ok(block) => {
				println!("Processed slot {}.", slot);
				block
			},
			Err(e) => {
				eprintln!("Failed to process slot {} after retries: {:?}", slot, e);
				return;
			}
		};

		println!(
				"Inspecting {} transactions from slot {}",
				block.transactions.as_ref().unwrap().len(),
				slot
		);

		let mut tree = match classify_block(slot, block, self.args.filter_transaction.clone()) {
				Ok(tree) => tree,
				Err(err) => {
						eprintln!("Failed to classify block: {:?}", err);
						return;
				}
		};

		let block_id = tree.root();

		label_tree(&mut tree);

		post_process(
				PostProcessConfig {
						retain_votes: false,
						remove_empty_transactions: true,
						cluster_jito_bundles: true,
				},
				&mut tree,
		);

		let block_documents = match database::document_builder::build_block_documents(&tree, block_id) {
				Ok(doc) => doc,
				Err(err) => {
						eprintln!("Failed to build block documents: {:?}", err);
						return;
				}
		};

		// Write block to beta DB (Should not be writing to prod with this CLI tool)
		if let Some(mongo_uri) = &self.args.mongo_uri {
				let rt = tokio::runtime::Runtime::new().unwrap();

				rt.block_on(async {
						let client = match MongoDBClient::new(MongoDBClientConfig {
								uri: mongo_uri.clone(),
								stage: MongoDBStage::Beta,
						})
						.await
						{
								Ok(client) => client,
								Err(err) => {
										eprintln!("Failed to create MongoDB client: {:?}", err);
										return;
								}
						};

						match client.write_block_documents(block_documents).await {
								Ok(_) => {}
								Err(err) => {
										eprintln!("Failed to write block documents: {:?}", err);
								}
						}
				});
		}

		if self.args.write_tree && tree.num_children(block_id) > 0 {
				self.write_tree_results(&tree);
		}
	}

	async fn process_epoch_async(&self, epoch: u64) {
		let epoch_schedule = match self.rpc_client.get_epoch_schedule() {
			Ok(schedule) => schedule,
			Err(err) => {
				eprintln!("Failed to get epoch schedule: {:?}", err);
				return;
			}
		};

		let first_slot = epoch_schedule.get_first_slot_in_epoch(epoch);
		let last_slot = epoch_schedule.get_last_slot_in_epoch(epoch);

		println!("Processing epoch {} (slots {} to {})", epoch, first_slot, last_slot);

		// Process each slot concurrently. Each slot is processed via a blocking task that runs
    // our retryable slot processing function.
		// Wrap the args in an Arc instead of cloning
		let args = Arc::new(self.args.clone());

		stream::iter(first_slot..=last_slot)
			.map(move |slot| {
				let args = Arc::clone(&args);  // This only clones the Arc pointer, not the data
				task::spawn_blocking(move || {
					let inspector = Inspector::new(InspectArgs { 
						slot: Some(slot),
						filter_transaction: (*args).filter_transaction.clone(),
						write_tree: (*args).write_tree,
						rpc_url: (*args).rpc_url.clone(),
						mongo_uri: (*args).mongo_uri.clone(),
						epoch: None,
					});
					inspector.process_slot_with_retry(slot)
				})
			})
			// Limit concurrency (here, up to 10 tasks concurrently).
			.buffer_unordered(10)
			.for_each(|res| async {
				if let Err(e) = res {
					eprintln!("Error processing slot: {:?}", e);
				}
			})
			.await;
	}

	fn write_tree_results(&self, tree: &impl std::fmt::Display) {
			// Create results directory if it doesn't exist
			fs::create_dir_all("results").expect("Failed to create results directory");

			// Get the current timestamp
			let timestamp = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.expect("Time went backwards")
					.as_secs();

			// Create the file path
			let file_path = format!("target/tree_results/{}_results.txt", timestamp);
			let file_path = Path::new(&file_path);

			// Create the directory if it doesn't exist
			if let Some(parent) = file_path.parent() {
					fs::create_dir_all(parent).expect("Failed to create directory");
			}

			// Write the tree to the file
			let mut file = File::create(&file_path).expect("Failed to create file");
			write!(file, "{}", tree).expect("Failed to write to file");

			println!("Results written to {}", file_path.display());
	}
}

pub async fn entry(args: InspectArgs) {
    let inspector = Inspector::new(args);

    match (inspector.args.slot, inspector.args.epoch) {
        (None, None) => {
            eprintln!("Error: Either --slot or --epoch must be provided");
            return;
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Cannot specify both --slot and --epoch");
            return;
        }
        (Some(slot), None) => inspector.process_slot_with_retry(slot),
        (None, Some(epoch)) => inspector.process_epoch_async(epoch).await,
    }
}

/// A helper function that wraps a closure with a retry policy using exponential backoff.
/// It will try the provided function (which should return a `Result`) until it either succeeds
/// or the retry policy gives up. Any error is printed before retrying.
fn retryable_request<T, E, F>(f: F) -> Result<T, retry::Error<E>>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Debug,
{
	// Configure an exponential backoff starting at 100ms
	retry(Exponential::from_millis(100).take(4), || {
			let result = f();
			if let Err(e) = &result {
					eprintln!("Encountered error, retrying: {:?}", e);
			}
			result
	})
}