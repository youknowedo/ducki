use std::{fs, path::Path};

use cursive::views::Dialog;
use inquire::Select;

use crate::{
    config::Config,
    deck::{progress::Progress, Log},
};

pub fn run(_deck_id: Option<String>, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => s.add_layer(Dialog::info("This command is not available in the TUI.")),
        None => terminal(_deck_id),
    }
}

fn terminal(_deck_id: Option<String>) {
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

    let progress_path = Path::new(deck_entry.path.as_str()).join(".progress.json");
    if !progress_path.exists() {
        panic!(
            "Progress file not found in path: {}",
            progress_path.display()
        );
    }
    let logs_path = Path::new(deck_entry.path.as_str()).join(".logs.json");
    if !logs_path.exists() {
        panic!("Logs file not found in path: {}", logs_path.display());
    }

    let mut new_progress = match fs::read_to_string(&progress_path) {
        Ok(contents) => match serde_json::from_str::<Progress>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                panic!("Could not parse deck file: {}", err);
            }
        },
        Err(err) => {
            panic!("Could not read deck file: {}", err);
        }
    };
    let mut new_logs = match fs::read_to_string(&logs_path) {
        Ok(contents) => match serde_json::from_str::<Vec<Log>>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                panic!("Could not parse deck file: {}", err);
            }
        },
        Err(err) => {
            panic!("Could not read deck file: {}", err);
        }
    };

    let last_log = new_logs.pop().unwrap();
    let last_card = last_log.last_card;

    // Replace last_progress.card where id == last_card.id with last_card
    let last_progress_card_index = new_progress
        .cards
        .iter_mut()
        .position(|card| card.id == last_card.id)
        .unwrap();
    new_progress.cards[last_progress_card_index] = last_card;

    // write
    match fs::write(progress_path, serde_json::to_string(&new_progress).unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not write progress file: {}", err);
        }
    }
    match fs::write(logs_path, serde_json::to_string(&new_logs).unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not write logs file: {}", err);
        }
    }
}
