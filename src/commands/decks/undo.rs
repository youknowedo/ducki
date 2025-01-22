use cursive::views::Dialog;
use inquire::Select;

use crate::{config::Config, deck::Deck};

pub fn run(_deck_id: Option<String>, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => s.add_layer(Dialog::info("This command is not available in the TUI.")),
        None => terminal(_deck_id),
    }
}

fn terminal(_deck_id: Option<String>) {
    let config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let deck_id = match _deck_id {
        Some(id) => id,
        None => match Select::new("Select a deck", config.decks.clone()).prompt() {
            Ok(selection) => selection.id,
            Err(err) => {
                panic!("Could not select deck: {}", err);
            }
        },
    };

    let mut deck = match Deck::get(deck_id.clone()) {
        Ok(deck) => deck,
        Err(err) => {
            panic!("Could not get deck: {}", err);
        }
    };

    let mut progress = match deck.progress() {
        Ok(progress) => progress,
        Err(err) => {
            panic!("Could not get progress: {}", err);
        }
    };
    let mut log = match deck.log() {
        Ok(log) => log,
        Err(err) => {
            panic!("Could not get log: {}", err);
        }
    };

    let last_log = log.entries.pop().unwrap();
    let last_card = last_log.last_card;

    // Replace last_progress.card where id == last_card.id with last_card
    let last_progress_card_index: usize = progress
        .cards
        .iter_mut()
        .position(|card| card.id == last_card.id)
        .unwrap();
    progress.cards[last_progress_card_index] = last_card;

    progress.save();
    log.save();
}
