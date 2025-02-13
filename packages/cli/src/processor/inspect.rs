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

#[derive(Args, Debug)]
pub struct InspectArgs {
    #[clap(long, help = "The slot to inspect.")]
    slot: u64,

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

pub fn entry(args: InspectArgs) {
    let rpc_client = RpcClient::new(args.rpc_url);

    let block = match rpc_client.get_block_with_config(
        args.slot,
        RpcBlockConfig {
            max_supported_transaction_version: Some(0),
            encoding: Some(UiTransactionEncoding::Base64),
            transaction_details: Some(TransactionDetails::Full),
            ..Default::default()
        },
    ) {
        Ok(block) => block,
        Err(err) => {
            eprintln!("Failed to get block: {:?}", err);
            return;
        }
    };

    if args.filter_transaction.is_none() {
        println!(
            "Inspecting {} transactions from slot {}",
            block.transactions.as_ref().unwrap().len(),
            args.slot
        );
    }

    let mut tree = match classify_block(args.slot, block, args.filter_transaction) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("Failed to classify block: {:?}", err);
            return;
        }
    };

		println!("TREE: \n {}", tree);

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
    if let Some(mongo_uri) = args.mongo_uri {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = match MongoDBClient::new(MongoDBClientConfig {
                uri: mongo_uri,
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

    if args.write_tree && tree.num_children(block_id) > 0 {
        // Create results directory if it doesn't exist
        fs::create_dir_all("results").expect("Failed to create results directory");

        // Get the current timestamp
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();

        // Create the file path
        let file_path = format!("target/tree_results/{}_results.txt", timestamp);
        let file_path = Path::new(&file_path);

        // Create the directory if it doesn't exist
        fs::create_dir_all(file_path.parent().unwrap()).expect("Failed to create directory");

        // Write the tree to the file
        let mut file = File::create(&file_path).expect("Failed to create file");
        write!(file, "{}", tree).expect("Failed to write to file");

        println!("Results written to {}", file_path.display());
    }
}
