use std::{
    fs::{create_dir_all, exists},
    path::Path,
};

use cursive::{
    view::Resizable,
    views::{Dialog, TextView},
};
use uuid::Uuid;

use crate::{
    deck::Deck,
    tui::init_deck::{select_id, InitData},
    util::write_temp_file_with_siv,
};

pub fn run(siv: &mut cursive::Cursive) {
    let default = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => std::path::PathBuf::from(""),
    };

    siv.add_layer(
        Dialog::around(
            cursive::views::EditView::new()
                .content(default.to_str().unwrap())
                .on_submit(on_submit)
                .fixed_width(50),
        )
        .title("Select path"),
    );
}

fn on_submit(s: &mut cursive::Cursive, path: &str) {
    s.pop_layer();
    let path = path.to_string();

    if let Err(err) = create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            s.add_layer(Dialog::info(format!("Could not create directory: {}", err)));
            return;
        }
    }

    match exists(Path::new(path.as_str()).join("deck.json")) {
        Ok(exists) => {
            if exists {
                s.add_layer(
                    Dialog::around(TextView::new("A deck already exists at this path."))
                        .title("Warning!")
                        .button("Change path", |s| {
                            s.pop_layer();
                            run(s);
                        })
                        .button("Overwrite", move |s| {
                            s.pop_layer();

                            save(s, path.as_str(), true, false);
                        }),
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
        Ok(_) => select_id::run(s, temp_file_id.clone()),
        Err(err) => {
            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
        }
    }
}
