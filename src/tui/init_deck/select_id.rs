use crate::config::get_config;
use crate::tui::init_deck::select_description::select_description;
use crate::util::{read_temp_file_with_siv, write_temp_file_with_siv};
use cursive::view::Resizable;
use cursive::views::{Dialog, TextView};
use std::path::Path;

use super::InitData;

pub fn select_id(siv: &mut cursive::Cursive, temp_file_id: String) {
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

                        fn save(
                            s: &mut cursive::Cursive,
                            temp_file_id: String,
                            id: &str,
                            overwrite_id_in_config: bool,
                        ) {
                            let mut data =
                                match read_temp_file_with_siv::<InitData>(s, &temp_file_id) {
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

                        let config = match get_config() {
                            Ok(config) => config,
                            Err(err) => {
                                s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                                return;
                            }
                        };
                        for deck in config.decks.iter() {
                            if deck.id == id {
                                s.add_layer(
                                    Dialog::around(TextView::new(
                                        "A deck with this id already exists.",
                                    ))
                                    .title("Warning!")
                                    .button("Change id", {
                                        let temp_file_id = temp_file_id.clone();
                                        move |s| {
                                            s.pop_layer();
                                            select_id(s, temp_file_id.clone());
                                        }
                                    })
                                    .button("Overwrite", {
                                        let temp_file_id = temp_file_id.clone();
                                        move |s| {
                                            s.pop_layer();
                                            save(s, temp_file_id.clone(), id.as_str(), true);
                                        }
                                    }),
                                );
                                return;
                            }
                        }

                        save(s, temp_file_id, id.as_str(), false);
                    }
                })
                .fixed_width(50),
        )
        .title("Select id"),
    );
}
