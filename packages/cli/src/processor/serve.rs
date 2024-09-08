use clap::{Args, ValueEnum};
use time_machine_api::{TimeMachineServer, TimeMachineServerConfig, TimeMachineStage};

#[derive(Args, Debug)]
pub struct ServeArgs {
    #[clap(short, long, env = "PORT", help = "Port to listen on.")]
    pub port: Option<u16>,

    #[clap(
        long,
        env = "RPC_RATE_LIMIT",
        help = "Rate limit for RPC requests per second.",
        default_value = "10"
    )]
    pub rpc_rate_limit: usize,

    #[clap(
        long,
        env = "RPC_URL",
        help = "RPC URL to use for fetching data.",
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    pub rpc_url: String,

    #[clap(
        long,
        env = "STAGE",
        help = "Stage of the Time Machine server.",
        default_value = "beta"
    )]
    pub stage: TimeMachineStageArg,

    #[clap(long, env = "MONGO_URI", help = "MongoDB URI to use for storing data.")]
    pub mongo_uri: String,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TimeMachineStageArg {
    Beta,
}

impl Into<TimeMachineStage> for TimeMachineStageArg {
    fn into(self) -> TimeMachineStage {
        match self {
            TimeMachineStageArg::Beta => TimeMachineStage::Beta,
        }
    }
}

pub async fn entry(args: ServeArgs) {
    let server_config = TimeMachineServerConfig {
        rpc_url: args.rpc_url,
        port: args.port.unwrap_or(8080),
        rpc_requests_per_second: args.rpc_rate_limit,
        stage: args.stage.into(),
        mongo_uri: args.mongo_uri,
    };

    let server = match TimeMachineServer::new(server_config).await {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to create Time Machine Server: {:?}", e);
            return;
        }
    };

    // Run server
    let result = server.serve().await;
    match result {
        Ok(_) => println!("Time Machine Server exited successfully"),
        Err(e) => eprintln!("Time Machine Server exited with error: {:?}", e),
    }
}
