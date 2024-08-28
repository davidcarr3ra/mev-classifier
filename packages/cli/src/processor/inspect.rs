use clap::Args;
use inspection::token_flows_from_ui_block;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};

#[derive(Args, Debug)]
pub struct InspectArgs {
    #[clap(long, help = "The slot to inspect.")]
    slot: u64,
}

pub fn entry(args: InspectArgs) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");

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

    match token_flows_from_ui_block(block) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to get token flows: {:?}", err);
        }
    }
}
