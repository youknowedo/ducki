use cursive::{
    align::HAlign,
    view::{Resizable, Scrollable},
    views::{Dialog, SelectView},
};

use crate::tui;

pub fn run(siv: &mut cursive::Cursive) {
    let config = crate::config::get_config();

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(config.decks.iter().map(|deck| deck.id.clone()));
    select.add_item("< Add new deck >", String::from(":add"));
    select.set_on_submit(select_deck);

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((20, 10))).title("Select a deck"));
}

fn select_deck(siv: &mut cursive::Cursive, id: &str) {
    if id == ":add" {
        siv.pop_layer();

        tui::init_deck::run(siv)
    } else {
        siv.pop_layer();
    }
}
