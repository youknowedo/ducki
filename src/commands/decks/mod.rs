use clap::{Parser, Subcommand};

pub mod add;
pub mod remove;
pub mod undo;
pub mod import;

#[derive(Parser, Debug, Clone)]
pub struct DeckArgs {
    pub deck_id: String,

    #[command(subcommand)]
    pub cmd: DeckCommands,
}
#[derive(Subcommand, Debug, Clone)]
pub enum DeckCommands {
    #[command(
        name = "add",
        alias = "a",
        about = "Add an existing deck to the config"
    )]
    Add(add::AddArgs),

    #[command(
        name = "import",
        alias = "i",
        about = "Import a deck from a file"
    )]
    Import(import::ImportArgs),

    #[command(name = "remove", alias = "rm", about = "Remove a deck from the config")]
    Remove(remove::RemoveArgs),

    #[command(name = "undo", alias = "u", about = "Undo the last action")]
    Undo,
}
