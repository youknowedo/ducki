use super::InitData;
use crate::config::{Config, DeckEntry};
use crate::tui::deck_select;
use cursive::views::Dialog;
use std::fs;
use std::path::PathBuf;

pub fn run(siv: &mut cursive::Cursive, data: InitData) {
    let path = PathBuf::from(data.path.clone());

    if let Err(err) = fs::create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            siv.add_layer(Dialog::info(format!("Could not create directory: {}", err)));
            return;
        }
    }

    let deck = data.deck;

    match deck.save() {
        Ok(_) => {}
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not write deck file: {}", err)));
            return;
        }
    }

    let mut config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    config.decks.push(DeckEntry {
        id: deck.id.clone(),
        path: data.path.clone(),
    });

    match config.save() {
        Ok(_) => deck_select::run(siv),
        Err(err) => {
            siv.add_layer(Dialog::info(format!(
                "Could not write config file: {}",
                err
            )));
            return;
        }
    }
}
