use clap::Parser;

mod config;
mod deck;
mod progress;

mod commands;
use commands::*;

mod util;

fn main() {
    let args = commands::Args::parse();

    run_command(args.cmd, &mut None);
}
