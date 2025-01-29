use crate::{config::Config, deck::Deck, tui::deck_select};

pub fn run(siv: &mut Option<&mut cursive::Cursive>) {
    let config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    match siv {
        Some(s) => {
            deck_select::run(s);
        }
        None => {}
    }

    for deck in config.decks {
        match siv {
            Some(s) => {
                s.call_on_name("content", |v: &mut cursive::views::TextView| {
                    v.append(format!("{}\n", deck.id));
                });
            }
            None => {
                let deck = Deck::get(deck.id).unwrap();


                println!("{}: {}", deck.id, deck.cards.len());
            }
        }
    }
}
