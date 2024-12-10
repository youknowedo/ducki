use clap::Parser;

mod config;
mod progress;
mod deck;

mod commands;
use commands::*;

#[derive(Parser)]
#[command(author("Sigfredo"), version("v0.0.2"), about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

fn main() {
    let args = Args::parse();

    run_command(args.cmd);
}
