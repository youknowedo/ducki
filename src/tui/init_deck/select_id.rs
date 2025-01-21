use crate::config::Config;
use crate::tui::init_deck::select_description;
use cursive::view::Resizable;
use cursive::views::{Dialog, TextView};
use std::path::Path;

use super::InitData;

pub fn run(siv: &mut cursive::Cursive, data: InitData) {
    let default = Path::new(&data.path).file_name().unwrap().to_str().unwrap();

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_string())
                .on_submit({
                    let data = data.clone();

                    move |s: &mut cursive::Cursive, id: &str| {
                        let id = id.to_string();
                        s.pop_layer();
                        let mut data = data.clone();

                        data.deck.id = id.to_string();

                        let config = match Config::get() {
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
                                        let data = data.clone();
                                        move |s| {
                                            s.pop_layer();
                                            run(s, data.clone());
                                        }
                                    })
                                    .button("Overwrite", {
                                        let  data = data.clone();
                                        move |s| {
                                            s.pop_layer();

                                            let mut data = data.clone();
                                            data.overwrite_id_in_config = true;
                                            select_description::run(s, data.clone());
                                        }
                                    }),
                                );
                                return;
                            }
                        }

                        data.overwrite_id_in_config = false;
                        select_description::run(s, data.clone());
                    }
                })
                .fixed_width(50),
        )
        .title("Select id"),
    );
}
