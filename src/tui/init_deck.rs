use serde::{Deserialize, Serialize};

use crate::deck::Deck;

mod select_path;
use select_path::select_path;
mod save_deck;
mod select_description;
mod select_id;

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
