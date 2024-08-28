use clap::Parser;

mod entry;
mod processor;

fn main() {
    let args = entry::CLI::parse();

    entry::entry(args);
}
