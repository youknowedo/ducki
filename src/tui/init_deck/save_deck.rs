use super::InitData;
use crate::config::{save_config_with_siv, DeckEntry};
use crate::tui::deck_select;
use crate::util::read_temp_file_with_siv;
use cursive::views::Dialog;
use std::fs;
use std::path::PathBuf;

pub fn save_deck(siv: &mut cursive::Cursive, temp_data_path: String) {
    let data = match read_temp_file_with_siv::<InitData>(siv, &temp_data_path) {
        Ok(data) => data,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
            return;
        }
    };

    let path = PathBuf::from(data.path.clone());

    if let Err(err) = fs::create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            siv.add_layer(Dialog::info(format!("Could not create directory: {}", err)));
            return;
        }
    }

    let deck = data.deck;

    let deck_as_string = match serde_json::to_string(&deck) {
        Ok(json) => json,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not serialize deck: {}", err)));
            return;
        }
    };

    match fs::write(path.join("deck.json"), deck_as_string) {
        Ok(_) => {}
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not write deck file: {}", err)));
            return;
        }
    };

    let mut config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    config.decks.push(DeckEntry {
        id: deck.id.clone(),
        path: data.path.clone(),
    });

    save_config_with_siv(siv, config);

    deck_select::run(siv);
}
