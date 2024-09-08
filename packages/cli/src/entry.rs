use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

use crate::processor::{self, inspect::InspectArgs, serve::ServeArgs};

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Inspect {
        #[clap(flatten)]
        args: InspectArgs,
    },
    Serve {
        #[clap(flatten)]
        args: ServeArgs,
    },
}

pub fn entry(cli: CLI) {
    match cli.command {
        Command::Inspect { args } => processor::inspect::entry(args),
        Command::Serve { args } => entry_async(processor::serve::entry(args)),
    }
}

fn entry_async(entry_fn: impl std::future::Future<Output = ()>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { entry_fn.await });
}
