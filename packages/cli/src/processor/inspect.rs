use clap::Args;
use classification::{
    classify_transaction, ActionTree, BlockAction, ClassifiableTransaction, TransactionAction,
};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};

#[derive(Args, Debug)]
pub struct InspectArgs {
    #[clap(long, help = "The slot to inspect.")]
    slot: u64,
}

pub fn entry(args: InspectArgs) {
    let rpc_client = RpcClient::new(
        "https://mainnet.helius-rpc.com/?api-key=6c48bd6c-00b7-421a-aefb-7fb2bc042fa2",
    );

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

    let root_action = BlockAction::new(args.slot);
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
        let tx_action = TransactionAction::new(signature);
        let tx_id = tree.insert(block_id, tx_action.into());

        let c_txn = ClassifiableTransaction::new(v_txn, txn.meta.unwrap());
        classify_transaction(c_txn, &mut tree, tx_id);
    }

    if tree.num_children(block_id) > 0 {
        println!("{}", tree);
    }
}
