use clap::Parser;

mod config;
mod deck;
mod progress;

mod commands;
use commands::*;
use cursive::view::*;

mod util;

fn main() {
    let args = commands::Args::parse();

    let mut siv = cursive::default();

    siv.add_fullscreen_layer(
        cursive::views::TextView::new("")
            .with_name("content")
            .scrollable()
            .full_screen(),
    );

    run_command(args.cmd, &mut siv);
}
