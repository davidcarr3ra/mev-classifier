use clap::Parser;
use tracing_subscriber::EnvFilter;

mod entry;
mod processor;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = entry::CLI::parse();

    entry::entry(args);
    
}
