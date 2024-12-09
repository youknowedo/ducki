use std::path::Path;

use clap::Parser;
use inquire::{Confirm, Select};

#[derive(Parser, Debug, Clone)]
pub struct RemoveArgs {
    id: Option<String>,
}

pub fn run(args: RemoveArgs) {
    let mut config = crate::config::get_config();

    if config.decks.is_empty() {
        println!("No decks found.");
        return;
    }

    let id = match args.id {
        Some(id) => id,
        None => {
            let mut ids = Vec::new();
            for deck in &config.decks {
                ids.push(deck.id.clone());
            }

            if let Ok(id) = Select::new("Which deck do you want to remove?", ids).prompt() {
                id
            } else {
                panic!("Could not get ID for deck to remove.");
            }
        }
    };

    let path = config
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .unwrap()
        .path
        .clone();

    config.decks.retain(|deck| deck.id != id);

    crate::config::save_config(config);

    if !Path::new(&path).exists() {
        return;
    }
    
    if let Ok(remove_dir) = Confirm::new("Do you want to remove the deck directory as well?")
        .with_default(false)
        .prompt()
    {
        if !remove_dir {
            return;
        }
    } else {
        panic!("Failed to get confirmation for removing the deck directory.");
    }

    if let Err(err) = std::fs::remove_dir_all(&path) {
        if err.kind() == std::io::ErrorKind::NotFound {
            return;
        }

        panic!("Could not remove directory: {}", err);
    }
}
