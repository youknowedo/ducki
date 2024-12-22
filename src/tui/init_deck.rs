use std::{
    fs,
    path::{Path, PathBuf},
};

use cursive::{
    view::Resizable,
    views::{Dialog, TextView},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{get_config, save_config_with_siv, DeckEntry},
    deck::Deck,
    util::{read_temp_file_with_siv, write_temp_file_with_siv},
};

use super::deck_select;

#[derive(Serialize, Deserialize)]
struct InitData<'a> {
    path: String,
    overwrite_file: bool,
    overwrite_path_in_config: bool,
    overwrite_id_in_config: bool,
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

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_str().unwrap())
                .on_submit(move |s, path| {
                    s.pop_layer();
                    let path = path.to_string();

                    if let Err(err) = fs::create_dir_all(&path) {
                        if err.kind() != std::io::ErrorKind::AlreadyExists {
                            s.add_layer(Dialog::info(format!(
                                "Could not create directory: {}",
                                err
                            )));
                            return;
                        }
                    }

                    fn save(
                        s: &mut cursive::Cursive,
                        path: &str,
                        overwrite_file: bool,
                        overwrite_path_in_config: bool,
                    ) {
                        let data = InitData {
                            path: path.to_string(),
                            overwrite_file,
                            overwrite_path_in_config,
                            overwrite_id_in_config: false,
                            deck: Deck::default(),
                        };

                        let temp_file_id = Uuid::new_v4().to_string();
                        match write_temp_file_with_siv(s, &temp_file_id, &data) {
                            Ok(_) => select_id(s, temp_file_id.clone()),
                            Err(err) => {
                                s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                            }
                        }
                    }

                    match fs::exists(Path::new(path.as_str()).join("deck.json")) {
                        Ok(exists) => {
                            if exists {
                                s.add_layer(
                                    Dialog::around(TextView::new(
                                        "A deck already exists at this path.",
                                    ))
                                    .title("Warning!")
                                    .button("Change path", |s| {
                                        s.pop_layer();
                                        select_path(s);
                                    })
                                    .button(
                                        "Overwrite",
                                        move |s| {
                                            s.pop_layer();

                                            save(s, path.as_str(), true, false);
                                        },
                                    ),
                                );
                                return;
                            }

                            save(s, path.as_str(), false, false);
                        }
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                            return;
                        }
                    }
                })
                .fixed_width(50),
        )
        .title("Select path"),
    );
}

fn select_id(siv: &mut cursive::Cursive, temp_file_id: String) {
    let data = match read_temp_file_with_siv::<InitData>(siv, &temp_file_id) {
        Ok(data) => data,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
            return;
        }
    };
    let default = Path::new(&data.path).file_name().unwrap().to_str().unwrap();

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_string())
                .on_submit({
                    let temp_file_id = temp_file_id.clone();
                    move |s: &mut cursive::Cursive, id: &str| {
                        let id = id.to_string();
                        let temp_file_id = temp_file_id.clone();
                        s.pop_layer();
                        let mut data = match read_temp_file_with_siv::<InitData>(s, &temp_file_id) {
                            Ok(data) => data,
                            Err(err) => {
                                s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                                return;
                            }
                        };

                        fn save(s: &mut cursive::Cursive, id: &str, overwrite_id_in_config: bool) {
                            let mut data = match read_temp_file_with_siv::<InitData>(s, id) {
                                Ok(data) => data,
                                Err(err) => {
                                    s.add_layer(Dialog::info(format!(
                                        "Something went wrong: {}",
                                        err
                                    )));
                                    return;
                                }
                            };

                            data.overwrite_id_in_config = overwrite_id_in_config;
                            data.deck.id = id.to_string();

                            match write_temp_file_with_siv(s, id, &data) {
                                Ok(_) => select_description(s, id.to_string()),
                                Err(err) => {
                                    s.add_layer(Dialog::info(format!(
                                        "Something went wrong: {}",
                                        err
                                    )));
                                }
                            }
                        }

                        data.deck.id = id.to_string();

                        let config = get_config();
                        for deck in config.decks.iter() {
                            if deck.id == id {
                                s.add_layer(
                                    Dialog::around(TextView::new(
                                        "A deck with this id already exists.",
                                    ))
                                    .title("Warning!")
                                    .button("Change id", move |s| {
                                        s.pop_layer();
                                        select_id(s, temp_file_id.clone());
                                    })
                                    .button(
                                        "Overwrite",
                                        move |s| {
                                            s.pop_layer();

                                            save(s, id.as_str(), true);
                                        },
                                    ),
                                );
                                return;
                            }
                        }

                        save(s, id.as_str(), false);
                    }
                })
                .fixed_width(50),
        )
        .title("Select id"),
    );
}

fn select_description(siv: &mut cursive::Cursive, temp_file_id: String) {
    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .on_submit(move |s, description| {
                    s.pop_layer();
                    let mut data = match read_temp_file_with_siv::<InitData>(s, &temp_file_id) {
                        Ok(data) => data,
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                            return;
                        }
                    };

                    data.deck.description = description.to_string();

                    match write_temp_file_with_siv(s, &temp_file_id, &data) {
                        Ok(_) => save_deck(s, temp_file_id.clone()),
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

    let mut config = get_config();

    config.decks.push(DeckEntry {
        id: deck.id.clone(),
        path: data.path.clone(),
    });

    save_config_with_siv(siv, config);

    deck_select::run(siv);
}
