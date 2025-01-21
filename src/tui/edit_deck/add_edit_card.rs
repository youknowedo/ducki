use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, DummyView, EditView, LinearLayout, TextView},
    Cursive,
};
use uuid::Uuid;

use crate::deck::{Card, Deck};

pub fn add(siv: &mut Cursive, deck_id: String) {
    _add_edit_card(siv, deck_id, None);
}

pub fn edit(siv: &mut Cursive, deck_id: String, card_id: String) {
    _add_edit_card(siv, deck_id, Some(card_id));
}

fn _add_edit_card(siv: &mut Cursive, deck_id: String, card_id: Option<String>) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let deck_entry = match config.decks.iter().find(|deck| deck.id == deck_id.clone()) {
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

    let default: Option<Card> = match card_id.clone() {
        Some(id) => deck.cards.iter().find(|card| card.id == id).cloned(),
        None => None,
    };

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Front:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(match default.clone() {
                            Some(card) => card.front.clone(),
                            None => "".to_string(),
                        })
                        .with_name("front")
                        .fixed_width(50),
                )
                .child(DummyView)
                .child(TextView::new("Back:").fixed_width(50))
                .child(
                    EditView::new()
                        .content(match default.clone() {
                            Some(card) => card.back.clone(),
                            None => "".to_string(),
                        })
                        .with_name("back")
                        .fixed_width(50),
                ),
        )
        .title("Select path")
        .button("Cancel", {
            let deck_id = deck_id.clone();
            move |siv| {
                siv.pop_layer();

                super::run(siv, deck_id.clone());
            }
        })
        .button(
            match default.clone() {
                Some(_) => "Save",
                None => "Add",
            },
            {
                let deck_id = deck_id.clone();
                let is_edit = match default {
                    Some(_) => true,
                    None => false,
                };
                let id = match card_id.clone() {
                    Some(id) => id,
                    None => Uuid::new_v4().to_string(),
                };

                move |siv| {
                    let config = match crate::config::get_config() {
                        Ok(config) => config,
                        Err(err) => panic!("Could not get config: {}", err),
                    };

                    let front = siv
                        .call_on_name("front", |view: &mut EditView| view.get_content())
                        .unwrap()
                        .to_string();
                    let back = siv
                        .call_on_name("back", |view: &mut EditView| view.get_content())
                        .unwrap()
                        .to_string();

                    let card = Card {
                        id: id.clone(),
                        front,
                        back,
                    };

                    let deck_entry =
                        match config.decks.iter().find(|deck| deck.id == deck_id.clone()) {
                            Some(deck) => deck.clone(),
                            None => return,
                        };

                    let deck_path = std::path::Path::new(deck_entry.path.as_str());

                    let mut deck: Deck = match std::fs::read_to_string(deck_path.join("deck.json"))
                    {
                        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
                            Ok(deck) => deck,
                            Err(err) => {
                                siv.add_layer(Dialog::info(format!(
                                    "Could not read deck file: {}",
                                    err
                                )));
                                return;
                            }
                        },
                        Err(err) => {
                            siv.add_layer(Dialog::info(format!(
                                "Could not read deck file: {}",
                                err
                            )));
                            return;
                        }
                    };

                    deck.config = Some(config);

                    if is_edit {
                        deck.cards = deck
                            .cards
                            .iter()
                            .map(|c| {
                                if c.id == card.id {
                                    card.clone()
                                } else {
                                    c.clone()
                                }
                            })
                            .collect();
                    } else {
                        deck.cards.push(card);
                    }

                    let deck_json = serde_json::to_string_pretty(&deck).unwrap();
                    std::fs::write(deck_path.join("deck.json"), deck_json).unwrap();

                    siv.pop_layer();

                    super::run(siv, deck_id.clone());
                }
            },
        ),
    );
}
