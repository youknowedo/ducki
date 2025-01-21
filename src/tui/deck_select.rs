use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, DummyView, LinearLayout, SelectView},
};

use crate::{config::save_config_with_siv, tui};

use super::{edit_deck, study};

pub fn run(siv: &mut cursive::Cursive) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump()
        .with_name("select");

    let mut select_mut = select.get_mut();
    select_mut.add_all_str(config.decks.iter().map(|deck| deck.id.clone()));
    select_mut.add_item("< Add new deck >", String::from(":add"));
    select_mut.set_on_submit(select_deck);

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select.scrollable().fixed_size((20, 10)))
                .child(DummyView)
                .child(
                    LinearLayout::vertical()
                        .child(Button::new("Edit", edit))
                        .child(Button::new("Delete", delete_deck)),
                ),
        )
        .title("Select a deck"),
    );
}

fn select_deck(siv: &mut cursive::Cursive, id: &str) {
    if id == ":add" {
        siv.pop_layer();

        tui::init_deck::run(siv, None)
    } else {
        siv.pop_layer();

        study::run(siv, id.to_string());
    }
}

fn edit(siv: &mut cursive::Cursive) {
    let id = match siv
        .call_on_name("select", |view: &mut SelectView| view.selection())
        .unwrap()
    {
        Some(id) => id,
        None => {
            siv.add_layer(Dialog::info("No deck selected"));
            return;
        }
    }.to_string();

    siv.pop_layer();

    edit_deck::run(siv, id);
}

fn delete_deck(siv: &mut cursive::Cursive) {
    let mut config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let id = match siv
        .call_on_name("select", |view: &mut SelectView| view.selection())
        .unwrap()
    {
        Some(id) => id,
        None => {
            siv.add_layer(Dialog::info("No deck selected"));
            return;
        }
    }.to_string();

    siv.pop_layer();

    config.decks.retain(|deck| deck.id != id);

    save_config_with_siv(siv, config);

    run(siv);
}
