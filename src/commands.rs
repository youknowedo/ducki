use clap::{command, Parser, Subcommand};

mod decks;
use decks::{DeckArgs, DeckCommands};

mod add;
use add::AddArgs;
mod init;
use init::InitArgs;
mod remove;
use remove::RemoveArgs;
mod list;
mod tui;

mod study;
use study::StudyArgs;

#[derive(Parser)]
#[command(author("Sigfredo"), version("v0.0.2"), about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
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

pub fn run_command(cmd: Option<Commands>) {
    match cmd {
        None => tui::run(),
        Some(cmd) => match cmd {
            Commands::List => list::run(),
            Commands::Init(args) => init::run(args),
            Commands::Add(args) => add::run(args),
            Commands::Remove(args) => remove::run(args),
            Commands::Deck(args) => match args.cmd {
                DeckCommands::Add(sub_args) => decks::add::run(Some(args.deck_id), sub_args),
                DeckCommands::Remove(sub_args) => decks::remove::run(Some(args.deck_id), sub_args),
                DeckCommands::Undo => decks::undo::run(Some(args.deck_id)),
            },

            Commands::Study(args) => study::run(args),
        },
    }
}
