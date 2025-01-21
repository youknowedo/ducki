use std::fs;

use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, TextView},
};

use crate::deck::Deck;

pub fn run(siv: &mut cursive::Cursive, deck_id: String) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let deck_entry = match config.decks.iter().find(|deck| deck.id == deck_id.clone()) {
        Some(deck) => deck.clone(),
        None => return,
    };

    let deck_path = std::path::Path::new(deck_entry.path.as_str());

    let deck: Deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
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
                let config = match crate::config::get_config() {
                    Ok(config) => config,
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not get config: {}", err)));
                        return;
                    }
                };

                let deck_entry = match config.decks.iter().find(|deck| deck.id == deck_id.clone()) {
                    Some(deck) => deck.clone(),
                    None => {
                        s.add_layer(Dialog::info("Could not find deck in config"));
                        return;
                    }
                };

                let deck_path = std::path::Path::new(deck_entry.path.as_str());

                let mut deck: Deck = match fs::read_to_string(deck_path.join("deck.json")) {
                    Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
                        Ok(deck) => deck,
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
                            return;
                        }
                    },
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not read deck file: {}", err)));
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

                let deck_as_string = match serde_json::to_string(&deck) {
                    Ok(json) => json,
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not serialize deck: {}", err)));
                        return;
                    }
                };

                match fs::write(deck_path.join("deck.json"), deck_as_string) {
                    Ok(_) => {}
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Could not write deck file: {}", err)));
                        return;
                    }
                };

                s.pop_layer();
            }
        }),
    );
}
