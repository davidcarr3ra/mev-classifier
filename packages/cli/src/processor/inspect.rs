use clap::{Args, ValueEnum};
use classifier_handler::classify_block;
use inspection::database::mongo_client::{MongoDBClient, MongoDBClientConfig, MongoDBStage};
use inspection::filtering::{post_process, PostProcessConfig};
use inspection::{database, label_tree};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_program::epoch_schedule::EpochSchedule as ProgramEpochSchedule;
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::{Arc, Mutex};
use std::num::NonZeroU32;
use retry::{retry, delay::Exponential};
use futures::stream::{self, StreamExt};
use tokio::task;
use reqwest::Client;
use actions::serialize_block_flat;
use serde_json::Value;
use urlencoding::encode;
use ratelimit_meter::{DirectRateLimiter, GCRA};

#[derive(ValueEnum, Debug, Clone)]
pub enum DatabaseType {
    Clickhouse,
    Mongo,
}

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

    #[clap(long, help = "ClickHouse URI to use for writing data.")]
    clickhouse_uri: Option<String>,

    #[clap(long, help = "ClickHouse username to use for writing data.")]
    clickhouse_username: Option<String>,

    #[clap(long, help = "ClickHouse password to use for writing data.")]
    clickhouse_password: Option<String>,

    #[clap(long, value_enum, default_value = "clickhouse", help = "Database to use: clickhouse or mongo")]
    db: DatabaseType,
}


lazy_static::lazy_static! {
    // Limit to 40 requests per 10 seconds
    // https://solana.com/docs/references/clusters#mainnet-beta-rate-limits
    
    // 15 requests per second: QuickNode Free Tier
    // https://www.quicknode.com/pricing
    static ref RPC_RATE_LIMITER: Mutex<DirectRateLimiter<GCRA>> =
        Mutex::new(DirectRateLimiter::new(NonZeroU32::new(12).unwrap(), Duration::from_secs(1)));
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

    /// Dispatches the slot processing to the appropriate database function based on the CLI flag.
    fn process_slot_with_retry(&self, slot: u64, epoch: Option<u64>) {
        match self.args.db {
            DatabaseType::Clickhouse => self.process_slot_with_retry_clickhouse(slot, epoch),
            DatabaseType::Mongo => self.process_slot_with_retry_mongo_db(slot),
        }
    }

    /// Asynchronously uploads a single block (in JSONEachRow format) to ClickHouse.
    async fn upload_block_to_clickhouse(clickhouse_url: &str, username: &str, password: &str, block: Value) -> Result<(), Box<dyn std::error::Error>> {
        // The ClickHouse query to insert one row using JSONEachRow format.
        let query = "INSERT INTO default.time_machine_v1 FORMAT JSONEachRow";
        // Build the full URL by URL-encoding the query.
        let full_url = format!("{}?query={}", clickhouse_url, encode(query));

        let client = Client::new();
        // Convert the block to a JSON string.
        let body = block.to_string();

        let response = client
            .post(&full_url)
            .basic_auth(username, Some(password))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Successfully uploaded block to ClickHouse.");
            Ok(())
        } else {
            Err(format!("ClickHouse insertion failed: HTTP {}", response.status()).into())
        }
    }

    /// Process a slot, classify its block, serialize it as a flattened JSON, and upload it to ClickHouse.
    fn process_slot_with_retry_clickhouse(&self, slot: u64, epoch: Option<u64>) {
        println!("Processing slot {} with retry into clickhouse.", slot);

        let block = match retryable_request(|| {
            rate_limited_rpc_call(|| {
                self.rpc_client.get_block_with_config(
                    slot,
                    RpcBlockConfig {
                        max_supported_transaction_version: Some(0),
                        encoding: Some(UiTransactionEncoding::Base64),
                        transaction_details: Some(TransactionDetails::Full),
                        ..Default::default()
                    },
                )
            })
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

        // Determine the epoch: use the provided one, or infer it if None.
        let epoch = match epoch {
            Some(e) => e,
            None => {
                let schedule = match retryable_request(|| self.rpc_client.get_epoch_schedule()) {
                    Ok(schedule) => {
                        println!("Retrieved epoch schedule for slot {}.", slot);
                        schedule
                    },
                    Err(e) => {
                        eprintln!("Failed to retrieve epoch schedule for slot {} after retries: {:?}", slot, e);
                        return;
                    }
                };
                let inferred_epoch = Self::infer_epoch_from_slot(slot, &schedule);
                println!("Inferred epoch {} for slot {}.", inferred_epoch, slot);
                inferred_epoch
            }
        };

        println!(
            "Inspecting {} transactions from slot {} (epoch {})",
            block.transactions.as_ref().unwrap().len(),
            slot,
            epoch
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

        // Serialize the block into a flat JSON structure.
        let flattened_block: Value = serialize_block_flat(&tree, block_id, epoch);

        // If a ClickHouse URI is provided, upload the flattened block.
        if let Some(clickhouse_uri) = &self.args.clickhouse_uri {

            let username = &self.args.clickhouse_username.as_deref().expect("ClickHouse username must be set");
            let password = &self.args.clickhouse_password.as_deref().expect("ClickHouse password must be set");

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match Self::upload_block_to_clickhouse(clickhouse_uri, username, password, flattened_block).await {
                    Ok(_) => println!("Uploaded slot {} to ClickHouse for epoch {}.", slot, epoch),
                    Err(err) => eprintln!("Failed to upload slot {} to ClickHouse: {:?}", slot, err),
                }
            });
        }
        
        // Optionally, if writing the tree results is enabled.
        if self.args.write_tree && tree.num_children(block_id) > 0 {
            self.write_tree_results(&tree);
        }
    }

    /// Helper function to infer the epoch for a given slot from the epoch schedule.
    fn infer_epoch_from_slot(slot: u64, schedule: &ProgramEpochSchedule) -> u64 {
        // If warmup is enabled and the slot is before the first normal slot, calculate using the warmup epochs.
        if schedule.warmup && slot < schedule.first_normal_slot {
            let mut epoch = 0;
            let mut epoch_start = 0;
            loop {
                let epoch_length = if epoch < schedule.first_normal_epoch {
                    // In warmup, each epoch length grows (typically doubling) until first_normal_epoch.
                    schedule.first_normal_slot >> (schedule.first_normal_epoch - epoch)
                } else {
                    schedule.slots_per_epoch
                };
                if epoch_start + epoch_length > slot {
                    break epoch;
                }
                epoch_start += epoch_length;
                epoch += 1;
            }
        } else {
            // For slots after warmup, use the standard formula.
            schedule.first_normal_epoch + ((slot - schedule.first_normal_slot) / schedule.slots_per_epoch)
        }
    }

	fn process_slot_with_retry_mongo_db(&self, slot: u64) {
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
		// let last_slot = epoch_schedule.get_last_slot_in_epoch(epoch);
        let last_slot = first_slot + 10000;

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
                        clickhouse_uri: (*args).clickhouse_uri.clone(),
                        clickhouse_username: (*args).clickhouse_username.clone(),
                        clickhouse_password: (*args).clickhouse_password.clone(),
                        db: (*args).db.clone(),
					});
					inspector.process_slot_with_retry(slot, Some(epoch))
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
    let inspector = Inspector::new(args.clone());

    match (inspector.args.slot, inspector.args.epoch) {
        (None, None) => {
            eprintln!("Error: Either --slot or --epoch must be provided");
            return;
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Cannot specify both --slot and --epoch");
            return;
        }
        (Some(slot), None) => inspector.process_slot_with_retry(slot, None),
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
	retry(Exponential::from_millis(100).take(2), || {
			let result = f();
			if let Err(e) = &result {
					eprintln!("Encountered error, retrying: {:?}", e);
			}
			result
	})
}

/// wraps an RPC call so that it first checks with the rate limiter before proceeding. 
///This minimal loop sleeps briefly until a “token” is available
fn rate_limited_rpc_call<T, E, F>(f: F) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Debug,
{
    loop {
        // Lock the rate limiter and check if a token is available.
        if RPC_RATE_LIMITER.lock().unwrap().check().is_ok() {
            break;
        } else {
            // Sleep a short duration (adjust as needed) before trying again.
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    // Proceed with the original RPC call.
    f()
}