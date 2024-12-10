use std::{fs, path::{Path, PathBuf}};

use clap::Parser;
use inquire::Select;

use crate::{config::get_config, deck::{Card, Deck}};

#[derive(Parser, Debug, Clone)]
pub struct ImportArgs {
    file: PathBuf,
}

pub fn run(_deck_id: Option<String>, args: ImportArgs) {
    let config = get_config();

    let deck_id = match _deck_id {
        Some(id) => {
            if !config.decks.iter().any(|deck| deck.id == id) {
                panic!("Deck not found: {}", id);
            }

            id
        }
        None => match Select::new("Select a deck", config.decks.clone()).prompt() {
            Ok(selection) => selection.id,
            Err(err) => {
                panic!("Could not select deck: {}", err);
            }
        },
    };

    let deck_entry = config.decks.iter().find(|deck| deck.id == deck_id).unwrap();

    // Check if the deck is in path
    let deck_path = Path::new(deck_entry.path.as_str()).join("deck.json");
    if !deck_path.exists() {
        panic!("Deck not found in path: {}", deck_path.display());
    }

    let mut  new_deck = match fs::read_to_string(&deck_path) {
        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
            Ok(mut deck) => {
                deck.config = Some(&config);

                deck
            }
            Err(err) => {
                panic!("Could not parse deck file: {}", err);
            }
        },
        Err(err) => {
            panic!("Could not read deck file: {}", err);
        }
    };

    // Import file using csv
    let imported_cards: Vec<Card> = match csv::Reader::from_path(args.file) {
        Ok(mut file) => file.deserialize().map(|result| match  result {
            Ok(card) => card,
            Err(err) => {
                panic!("Could not parse card: {}", err);
            }
            
        }).collect(),
        Err(err) => {
            panic!("Could not read file: {}", err);
        }
    };

    new_deck.cards.extend(imported_cards);

    match fs::write(deck_path, serde_json::to_string_pretty(&new_deck).unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not write deck file: {}", err);
        }
    }
}
