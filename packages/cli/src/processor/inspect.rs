use actions::Block;
use clap::Args;
use classifier_core::{ActionTree, ClassifiableTransaction};
use classifier_handler::classify_transaction;
use inspection::filtering::{post_process, PostProcessConfig};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Args, Debug)]
pub struct InspectArgs {
    #[clap(long, help = "The slot to inspect.")]
    slot: u64,

    #[clap(long, help = "Filter transactions by signature.")]
    filter_transaction: Option<String>,

    #[clap(long, help = "Write tree to results folder", default_value = "true")]
    write_tree: bool,

    #[clap(
        long,
        help = "RPC URL to use for fetching data.",
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    rpc_url: String,
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

    if block.transactions.is_none() {
        eprintln!("No transactions in block data");
        return;
    }

    println!(
        "Inspecting {} transactions from slot {}",
        block.transactions.as_ref().unwrap().len(),
        args.slot
    );

    let root_action = Block::new(args.slot);
    let mut tree = ActionTree::new(root_action.into());
    let block_id = tree.root();

    for txn in block.transactions.unwrap() {
        let v_txn = txn.transaction.decode().unwrap();

        if txn.meta.is_none() {
            eprintln!("No transaction meta data");
            continue;
        }

        // Setup new tree for each transaction.
        // In the future, this will be entire block, or even multiple blocks.
        // Either way, the plumbing is there for this to easily happen.
        let signature = v_txn.signatures.first().unwrap().clone();

        if let Some(filter) = &args.filter_transaction {
            if signature.to_string() != *filter {
                continue;
            }
        }

        let tx_action = actions::Transaction::new(signature);
        let tx_id = tree.insert(block_id, tx_action.into());

        let c_txn = ClassifiableTransaction::new(v_txn, txn.meta.unwrap());

        match classify_transaction(c_txn, &mut tree, tx_id) {
            Ok(_) => {}
            Err(err) => {
                eprintln!(
                    "Failed to classify transaction: {:?}, signature: {}",
                    err, signature
                );
            }
        }
    }

    post_process(
        PostProcessConfig {
            retain_votes: false,
            remove_empty_transactions: true,
            cluster_jito_bundles: true,
        },
        &mut tree,
    );

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
        let file_path = format!("results/{}_results.txt", timestamp);

        // Write the tree to the file
        let mut file = File::create(&file_path).expect("Failed to create file");
        write!(file, "{}", tree).expect("Failed to write to file");

        println!("Results written to {}", file_path);
    }
}
