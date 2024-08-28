use clap::{Parser, Subcommand};

use crate::processor::{self, inspect::InspectArgs};

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
}

pub fn entry(cli: CLI) {
    match cli.command {
        Command::Inspect { args } => processor::inspect::entry(args),
    }
}
