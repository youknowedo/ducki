use add::AddArgs;
use clap::{Parser, Subcommand};

mod decks;
use decks::*;
mod cards;
use init::InitArgs;
use remove::RemoveArgs;
use study::StudyArgs;

#[derive(Parser, Debug, Clone)]
pub struct DeckArgs {
    #[command(subcommand)]
    cmd: DeckCommands,

    deck_id: Option<String>,
}
#[derive(Subcommand, Debug, Clone)]
pub enum DeckCommands {
    #[command(name = "add", alias = "a", about = "Add a card to a deck")]
    Add(cards::add::AddArgs),

    #[command(name = "remove", alias = "rm", about = "Remove a card from a deck")]
    Remove(cards::remove::RemoveArgs),
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

    #[command(name = "deck", alias = "d", about = "Manage cards in a deck")]
    Deck(DeckArgs),
}

pub fn run_command(cmd: Commands) {
    match cmd {
        Commands::List => list::run(),
        Commands::Init(args) => init::run(args),
        Commands::Add(args) => add::run(args),
        Commands::Remove(args) => remove::run(args),
        Commands::Study(args) => study::run(args),
        Commands::Deck(args) => match args.cmd {
            DeckCommands::Add(sub_args) => cards::add::run(args.deck_id, sub_args),
            DeckCommands::Remove(sub_args) => cards::remove::run(args.deck_id, sub_args),
        },
    }
}
