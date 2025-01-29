use clap::Parser;

mod config;
mod deck;

mod commands;
use commands::*;

mod tui;

fn main() {
    let args = commands::Args::parse();

    run_command(args.cmd, &mut None);
}
