use std::{fs, path::Path};

use clap::Parser;
use cursive::views::Dialog;
use inquire::{Select, Text};

use crate::{
    config::get_config,
    deck::{Card, Deck},
};

#[derive(Parser, Debug, Clone)]
pub struct AddArgs {
    #[clap(long = "id", short = 'i')]
    pub id: Option<String>,
    #[clap(long = "front", short = 'f')]
    pub front: Option<String>,
    #[clap(long = "back", short = 'b')]
    pub back: Option<String>,
}

pub fn run(_deck_id: Option<String>, args: AddArgs, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => s.add_layer(Dialog::info("This command is not available in the TUI.")),
        None => terminal(_deck_id, args),
    }
}

fn terminal(_deck_id: Option<String>, args: AddArgs) {
    let mut config = match crate::config::get_config() {
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
    let deck_path = Path::new(deck_entry.path.as_str()).join("deck.json");
    if !deck_path.exists() {
        panic!("Deck not found in path: {}", deck_path.display());
    }

    let id = match args.id {
        Some(id) => id,
        None => {
            let mut id_is_free = false;
            let mut id = String::new();

            while !id_is_free {
                id = match Text::new("Enter a unique ID for the card").prompt() {
                    Ok(id) => id,
                    Err(err) => {
                        panic!("Could not get ID for card: {}", err);
                    }
                }.trim().to_string();

                if id.is_empty() {
                    println!("ID cannot be empty. Please try again.");
                    continue;
                }

                id_is_free = match fs::read_to_string(&deck_path) {
                    Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
                        Ok(deck) => !deck.cards.iter().any(|card| card.id == id),
                        Err(err) => {
                            panic!("Could not parse deck file: {}", err);
                        }
                    },
                    Err(err) => {
                        panic!("Could not read deck file: {}", err);
                    }
                };

                if !id_is_free {
                    println!("ID is not unique. Please try again.");
                }
            }

            id
        }
    };

    let front = match args.front {
        Some(front) => front,
        None => match Text::new("Enter the front of the card").prompt() {
            Ok(front) => front,
            Err(err) => {
                panic!("Could not get front of card: {}", err);
            }
        },
    };

    let back = match args.back {
        Some(back) => back,
        None => match Text::new("Enter the back of the card").prompt() {
            Ok(back) => back,
            Err(err) => {
                panic!("Could not get back of card: {}", err);
            }
        },
    };

    let mut new_deck = match fs::read_to_string(&deck_path) {
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
    new_deck.cards.push(Card { id, front, back });

    match fs::write(&deck_path, serde_json::to_string_pretty(&new_deck).unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not write deck file: {}", err);
        }
    }
}
