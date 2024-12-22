use core::panic;
use std::{fs, path::PathBuf};

use cursive::{view::Resizable, views::Dialog};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{get_config, save_config_with_siv, DeckEntry},
    deck::Deck,
};

use super::deck_select;

#[derive(Serialize, Deserialize)]
struct InitData<'a> {
    path: String,
    deck: Deck<'a>,
}

pub fn run(siv: &mut cursive::Cursive) {
    select_path(siv);
}

fn select_path(siv: &mut cursive::Cursive) {
    let default = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => std::path::PathBuf::from(""),
    };

    let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
        Some(dirs) => dirs,
        None => {
            siv.add_layer(Dialog::info("Could not find project directories"));
            return;
        }
    };

    let cache_dir = project_dirs.cache_dir().to_path_buf();

    let temp_data_path = cache_dir
        .join(Uuid::new_v4().to_string())
        .to_str()
        .unwrap()
        .to_string();

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_str().unwrap())
                .on_submit(move |s, path| {
                    let path = std::path::PathBuf::from(path);
                    if path.exists() {
                        s.pop_layer();

                        let data = InitData {
                            path: path.to_str().unwrap().to_string(),
                            deck: Deck::default(),
                        };

                        match fs::exists(temp_data_path.as_str()) {
                            Ok(exists) => {
                                if !exists {
                                    match fs::create_dir_all(cache_dir.clone()) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            s.add_layer(Dialog::info(format!(
                                                "Something went wrong: {}",
                                                err
                                            )));
                                        }
                                    }
                                }
                            }
                            Err(_) => match fs::create_dir_all(cache_dir.clone()) {
                                Ok(_) => {}
                                Err(err) => {
                                    s.add_layer(Dialog::info(format!(
                                        "Something went wrong: {}",
                                        err
                                    )));
                                }
                            },
                        }

                        match fs::write(
                            temp_data_path.as_str(),
                            serde_json::to_string(&data).unwrap(),
                        ) {
                            Ok(_) => select_id(s, temp_data_path.clone()),
                            Err(err) => {
                                s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                            }
                        }
                    } else {
                        s.add_layer(Dialog::info("Path does not exist"));
                    }
                })
                .fixed_width(50),
        )
        .title("Select path"),
    );
}

fn select_id(siv: &mut cursive::Cursive, temp_data_path: String) {
    let data = match fs::read_to_string(temp_data_path.as_str()) {
        Ok(contents) => match serde_json::from_str::<InitData>(&contents) {
            Ok(data) => data,
            Err(err) => {
                panic!("Something went wrong: {}", err);
            }
        },
        Err(err) => {
            panic!("Something went wrong: {}", err);
        }
    };

    let path = PathBuf::from(data.path.clone());
    let default = path.file_name().unwrap().to_str().unwrap();

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_string())
                .on_submit(move |s, id| {
                    s.pop_layer();
                    let mut data = match fs::read_to_string(temp_data_path.as_str()) {
                        Ok(contents) => match serde_json::from_str::<InitData>(&contents) {
                            Ok(data) => data,
                            Err(err) => {
                                panic!("Something went wrong: {}", err);
                            }
                        },
                        Err(err) => {
                            panic!("Something went wrong: {}", err);
                        }
                    };

                    data.deck.id = id.to_string();

                    match fs::write(
                        temp_data_path.as_str(),
                        serde_json::to_string(&data).unwrap(),
                    ) {
                        Ok(_) => select_description(s, temp_data_path.clone()),
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                        }
                    }
                })
                .fixed_width(50),
        )
        .title("Select id"),
    );
}

fn select_description(siv: &mut cursive::Cursive, temp_data_path: String) {
    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .on_submit(move |s, description| {
                    s.pop_layer();
                    let mut data = match fs::read_to_string(temp_data_path.as_str()) {
                        Ok(contents) => match serde_json::from_str::<InitData>(&contents) {
                            Ok(data) => data,
                            Err(err) => {
                                panic!("Something went wrong: {}", err);
                            }
                        },
                        Err(err) => {
                            panic!("Something went wrong: {}", err);
                        }
                    };

                    data.deck.description = description.to_string();

                    match fs::write(
                        temp_data_path.as_str(),
                        serde_json::to_string(&data).unwrap(),
                    ) {
                        Ok(_) => save_deck(s, temp_data_path.clone()),
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                        }
                    }
                })
                .fixed_width(50),
        )
        .title("Select description"),
    );
}

fn save_deck(siv: &mut cursive::Cursive, temp_data_path: String) {
    let data = match fs::read_to_string(temp_data_path.as_str()) {
        Ok(contents) => match serde_json::from_str::<InitData>(&contents) {
            Ok(data) => data,
            Err(err) => {
                siv.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                return;
            }
        },
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

    let mut config = get_config();

    config.decks.push(DeckEntry {
        id: deck.id.clone(),
        path: data.path.clone(),
    });

    save_config_with_siv(siv, config);

    deck_select::run(siv);
}
