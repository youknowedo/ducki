use std::fs;

use clap::Parser;
use cursive::views::Dialog;
use inquire::{Confirm, Select, Text};

use crate::{config::Config, deck::Deck};

#[derive(Parser, Debug, Clone)]
pub struct RemoveArgs {
    pub id: Option<String>,
}

pub fn run(_deck_id: Option<String>, args: RemoveArgs, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => s.add_layer(Dialog::info("This command is not available in the TUI.")),
        None => terminal(_deck_id, args),
    }
}

fn terminal(_deck_id: Option<String>, args: RemoveArgs) {
    let config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

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
    let deck_path = deck_entry.path.join("deck.json");
    if !deck_path.exists() {
        panic!("Deck not found in path: {}", deck_path.display());
    }

    let mut deck = match fs::read_to_string(&deck_path) {
        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                panic!("Could not parse deck file: {}", err);
            }
        },
        Err(err) => {
            panic!("Could not read deck file: {}", err);
        }
    };

    let id = match args.id {
        Some(id) => id,
        None => match Text::new("Enter a unique ID for the card").prompt() {
            Ok(id) => id,
            Err(err) => {
                panic!("Could not get ID for card: {}", err);
            }
        },
    };

    deck.cards.retain(|card| card.id != id);

    if match Confirm::new("Are you sure you want to remove this card?").prompt() {
        Ok(confirmation) => confirmation,
        Err(err) => {
            panic!("Could not get confirmation: {}", err);
        }
    } {
        match deck.save() {
            Ok(_) => {
                println!("Card removed.");
            }
            Err(err) => {
                panic!("Could not save deck: {}", err);
            }
        }
    }
}
