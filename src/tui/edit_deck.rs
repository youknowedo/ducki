use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, DummyView, LinearLayout, SelectView},
};

use crate::deck::Deck;

mod add_edit_card;
mod edit_details;

pub fn run(siv: &mut cursive::Cursive, deck_id: String) {
    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump()
        .with_name("select");

    let deck = match Deck::get(deck_id.clone()) {
        Ok(deck) => deck,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not get deck: {}", err)));
            return;
        }
    };

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

    let mut deck = match Deck::get(deck_id.clone()) {
        Ok(deck) => deck,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not get deck: {}", err)));
            return;
        }
    };

    deck.cards.retain(|card| card.id != id);

    match deck.save() {
        Ok(_) => {}
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not save deck: {}", err)));
            return;
        }
    };

    run(siv, deck_id);
}
