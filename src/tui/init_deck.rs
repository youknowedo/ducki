use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, TextView},
};
use serde::{Deserialize, Serialize};

use crate::{config::Config, deck::Deck};

use super::deck_select;

#[derive(Serialize, Deserialize, Clone)]
struct InitData {
    path: String,
    overwrite_file: bool,
    overwrite_path_in_config: bool,
    overwrite_id_in_config: bool,
    deck: Deck,
}

pub fn run(siv: &mut cursive::Cursive, id: Option<String>) {
    let default_path = match std::env::current_dir() {
        Ok(path) => match id {
            Some(id) => path.join(id.clone()),
            None => path,
        },
        Err(_) => std::path::PathBuf::from(""),
    };

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Path:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(default_path.to_str().unwrap())
                        .on_edit(|siv, path, _| {
                            siv.call_on_name("id", |view: &mut EditView| {
                                // FIXME: Only set the content if the user has not edited the ID field
                                view.set_content(
                                    Path::new(path)
                                        .file_name()
                                        .unwrap_or(OsStr::new(""))
                                        .to_str()
                                        .unwrap(),
                                );
                            });
                        })
                        .with_name("path")
                        .fixed_width(50),
                )
                .child(TextView::new("ID:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(
                            default_path
                                .file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_str()
                                .unwrap(),
                        )
                        .with_name("id")
                        .fixed_width(50),
                )
                .child(TextView::new("Description:").fixed_width(50))
                .child(EditView::new().with_name("description").fixed_width(50)),
        )
        .button("Save", {
            move |s| {
                let id = s
                    .call_on_name("id", |view: &mut EditView| view.get_content())
                    .unwrap();
                let path = s
                    .call_on_name("path", |view: &mut EditView| view.get_content())
                    .unwrap();
                let description = s
                    .call_on_name("description", |view: &mut EditView| view.get_content())
                    .unwrap();

                let deck_path = PathBuf::from(&path.to_string());

                // Create directories if they do not exist
                if let Err(err) = fs::create_dir_all(&deck_path) {
                    s.add_layer(Dialog::info(format!("Could not create directory: {}", err)));
                    return;
                }

                let deck = Deck::new(
                    id.to_string(),
                    deck_path.clone(),
                    description.to_string(),
                );

                match deck.save() {
                    Ok(_) => {}
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not save deck: {}", err)));
                        return;
                    }
                }

                let mut config = match Config::get() {
                    Ok(config) => config,
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not get config: {}", err)));
                        return;
                    }
                };

                config.decks.push(deck.to_entry());

                match config.save() {
                    Ok(_) => {}
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not save config: {}", err)));
                        return;
                    }
                }

                s.pop_layer();

                deck_select::run(s);
            }
        }),
    );
}
