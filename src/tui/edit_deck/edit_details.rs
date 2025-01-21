use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, TextView},
};

use crate::deck::Deck;

pub fn run(siv: &mut cursive::Cursive, deck_id: String) {
    let deck = match Deck::get(deck_id.clone()) {
        Ok(deck) => deck,
        Err(e) => {
            siv.add_layer(Dialog::info(format!("Could not get deck: {}", e)));
            return;
        }
    };

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("ID:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(deck.id.clone())
                        .with_name("id")
                        .fixed_width(50),
                )
                .child(TextView::new("Description:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(deck.description.clone())
                        .with_name("description")
                        .fixed_width(50),
                ),
        )
        .button("Cancel", {
            let deck_id = deck_id.clone();

            move |siv| {
                siv.pop_layer();

                super::run(siv, deck_id.clone());
            }
        })
        .button("Save", {
            let deck_id = deck_id.clone();

            move |s| {
                let mut deck = match Deck::get(deck_id.clone()) {
                    Ok(deck) => deck,
                    Err(e) => {
                        s.add_layer(Dialog::info(format!("Could not get deck: {}", e)));
                        return;
                    }
                };

                let id = s
                    .call_on_name("id", |view: &mut EditView| view.get_content())
                    .unwrap();
                let description = s
                    .call_on_name("description", |view: &mut EditView| view.get_content())
                    .unwrap();

                deck.id = id.to_string();
                deck.description = description.to_string();

                match deck.save() {
                    Ok(_) => {}
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not save deck: {}", err)));
                        return;
                    }
                }

                s.pop_layer();
            }
        }),
    );
}
