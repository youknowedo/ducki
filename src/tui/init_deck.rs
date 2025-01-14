use serde::{Deserialize, Serialize};

use crate::deck::Deck;

mod save_deck;
mod select_description;
mod select_id;
mod select_path;

#[derive(Serialize, Deserialize)]
struct InitData<'a> {
    path: String,
    overwrite_file: bool,
    overwrite_path_in_config: bool,
    overwrite_id_in_config: bool,
    deck: Deck<'a>,
}

pub fn run(siv: &mut cursive::Cursive, id: Option<String>) {
    select_path::run(siv, id);
}
