use clap::{command, Parser, Subcommand};

mod decks;
use decks::{DeckArgs, DeckCommands};

mod default;
mod add;
pub use add::AddArgs;
mod init;
pub use init::InitArgs;
mod remove;
pub use remove::RemoveArgs;
mod help;
mod list;

mod study;
use study::StudyArgs;

use crate::tui;

#[derive(Parser, Debug)]
#[command(author("Sigfredo"), version("v0.0.2"), about, long_about = None, disable_help_flag = true, disable_help_subcommand = true)] // Disable default help flag
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(name = "help", about = "Print help information")]
    Help,

    #[command(name = "list", alias = "ls", about = "List all decks")]
    List,

    #[command(
        name = "init",
        alias = "i",
        about = "Initialize a new deck and add it to the config"
    )]
    Init(InitArgs),

    #[command(
        name = "add",
        alias = "a",
        about = "Add an existing deck to the config"
    )]
    Add(AddArgs),

    #[command(name = "remove", alias = "rm", about = "Remove a deck from the config")]
    Remove(RemoveArgs),

    #[command(name = "study", alias = "s", about = "Study a deck")]
    Study(StudyArgs),

    #[command(
        name = "deck",
        alias = "d",
        about = "Manage cards in a deck",
        arg_required_else_help = true
    )]
    Deck(DeckArgs),
}

pub fn run_command(cmd: Option<Commands>, siv: &mut Option<&mut cursive::Cursive>) {
    match cmd {
        None => match siv {
            Some(s) => default::run(s),
            None => {
                let mut siv = cursive::default();

                tui::setup(&mut siv);

                default::run(&mut siv)
            }
        },
        Some(cmd) => match cmd {
            Commands::Help => help::run(siv),
            Commands::List => list::run(siv),
            Commands::Init(args) => init::run(args, siv),
            Commands::Add(args) => add::run(args, siv),
            Commands::Remove(args) => remove::run(args, siv),
            Commands::Deck(args) => match args.cmd {
                DeckCommands::Add(sub_args) => decks::add::run(Some(args.deck_id), sub_args, siv),
                DeckCommands::Remove(sub_args) => decks::remove::run(Some(args.deck_id), sub_args, siv),
                DeckCommands::Undo => decks::undo::run(Some(args.deck_id), siv),
            },

            Commands::Study(args) => study::run(args, siv),
        },
    }
}
