use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, DummyView, LinearLayout, SelectView},
};
use serde_json::json;

use crate::deck::Deck;

mod add_edit_card;
mod edit_details;

pub fn run(siv: &mut cursive::Cursive, deck_id: String) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump()
        .with_name("select");

    let deck_entry = match config.decks.iter().find(|deck| deck.id == deck_id) {
        Some(deck) => deck.clone(),
        None => return,
    };

    let deck_path = std::path::Path::new(deck_entry.path.as_str());

    let mut deck: Deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                siv.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
                return;
            }
        },
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
            return;
        }
    };

    deck.config = Some(config);

    let mut select_mut = select.get_mut();

    deck.cards
        .iter()
        .for_each(|card| select_mut.add_item(card.front.clone(), card.id.clone()));

    select_mut.add_item("< Add new card >", String::from(":add"));
    select_mut.set_on_submit({
        let deck_id = deck_id.clone();
        move |siv, id| select_card(siv, deck_id.clone(), id)
    });

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select.scrollable().fixed_size((20, 10)))
                .child(DummyView)
                .child(
                    LinearLayout::vertical()
                        .child(Button::new("Edit deck", {
                            let deck_id = deck_id.clone();
                            move |siv| edit_deck(siv, deck_id.clone())
                        }))
                        .child(Button::new("Delete", {
                            let deck_id = deck_id.clone();
                            move |s| delete_card(s, deck_id.clone())
                        }))
                        .child(Button::new("Back", |s| {
                            s.pop_layer();
                            super::deck_select::run(s)
                        })),
                ),
        )
        .title("Select a card"),
    );
}

fn select_card(siv: &mut cursive::Cursive, deck_id: String, id: &str) {
    if id == ":add" {
        siv.pop_layer();

        add_edit_card::add(siv, deck_id);
    } else {
        siv.pop_layer();

        add_edit_card::edit(siv, deck_id, id.to_string());
    }
}

fn edit_deck(siv: &mut cursive::Cursive, deck_id: String) {
    siv.pop_layer();

    edit_details::run(siv, deck_id);
}

fn delete_card(siv: &mut cursive::Cursive, deck_id: String) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let id = match siv
        .call_on_name("select", |view: &mut SelectView| view.selection())
        .unwrap()
    {
        Some(id) => id,
        None => {
            siv.add_layer(Dialog::info("No card selected"));
            return;
        }
    }
    .to_string();

    siv.pop_layer();

    let deck_entry = match config.decks.iter().find(|deck| deck.id == deck_id) {
        Some(deck) => deck.clone(),
        None => return,
    };

    let deck_path = std::path::Path::new(deck_entry.path.as_str());

    let mut deck: Deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                siv.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
                return;
            }
        },
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
            return;
        }
    };

    deck.config = Some(config);

    deck.cards.retain(|card| card.id != id);

    match std::fs::write(deck_path.join("deck.json"), json!(deck).to_string()) {
        Ok(_) => {}
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not write deck file: {}", err)));
            return;
        }
    };

    run(siv, deck_id);
}
