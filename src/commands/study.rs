use std::fs;

use chrono::Utc;
use clap::Parser;
use inquire::{Select, Text};
use rs_fsrs::{Card as FSRSCard, Rating as FSRSRating, FSRS};

use crate::config::Config;
use crate::deck::log::LogEntry;
use crate::deck::{
    progress::{Progress, ProgressCard, Rating},
    Deck,
};
use rand::seq::SliceRandom;

use crate::tui::study::run as tui;

#[derive(Parser, Debug, Clone)]
pub struct StudyArgs {
    id: String,
}

pub fn run(args: StudyArgs, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => tui(s, args.id),
        None => {
            match siv {
                Some(s) => s.quit(),
                None => {}
            }
            terminal(args)
        }
    }
}

fn terminal(args: StudyArgs) {
    let config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };
    let now = Utc::now();

    if config.decks.is_empty() {
        println!("No decks found in config.");
        return;
    }

    let deck_entry = match Some(args.id) {
        Some(id) => match config.decks.iter().find(|deck| deck.id == id) {
            Some(deck) => deck.clone(),
            None => {
                println!("Deck not found in config.");
                return;
            }
        },
        None => match Select::new("Select a deck to study", config.decks.clone()).prompt() {
            Ok(entry) => entry.clone(),
            Err(_) => {
                println!("Failed to get deck selection.");
                return;
            }
        },
    };
    let deck_path = deck_entry.path;

    let deck: Deck = match fs::read_to_string(deck_path.join("deck.json")) {
        Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
            Ok(deck) => deck,
            Err(err) => {
                panic!("Could not parse deck file: {}", err);
            }
        },
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                panic!("Deck file not found.");
            } else {
                panic!("Could not read deck file: {}", err);
            }
        }
    };

    let progress_path = deck_path.join(".progress.json");

    let mut progress: Progress = match fs::read_to_string(&progress_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(progress) => progress,
            Err(err) => {
                panic!("Could not parse progress file: {}", err);
            }
        },
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                match Progress::get(&deck) {
                    Ok(progress) => progress,
                    Err(err) => {
                        panic!("Could not get progress: {}", err);
                    }
                }
            } else {
                panic!("Could not read progress file: {}", err);
            }
        }
    };

    // Remove any cards that have been removed from the deck by checking ids
    progress.cards.retain(|progress_card| {
        deck.cards
            .iter()
            .find(|card| card.id == progress_card.id)
            .is_some()
    });

    // Get cards from progress whose due date is before now
    let mut cards = progress
        .cards
        .iter()
        .filter(|progress_card| progress_card.fsrs.due.clone() < now)
        .map(|progress_card| progress_card.clone())
        .collect::<Vec<_>>();

    // Add any new cards to the progress file
    for card in deck.cards.iter() {
        if progress
            .cards
            .iter()
            .find(|progress_card| progress_card.id == card.id)
            .is_none()
        {
            let progress_card = ProgressCard {
                id: card.id.clone(),
                fsrs: FSRSCard::default(),
            };

            progress.cards.push(progress_card.clone());
            cards.push(progress_card);
        }
    }

    let progress_json = serde_json::to_string_pretty(&progress).unwrap();
    fs::write(&progress_path, progress_json).unwrap();

    if cards.is_empty() {
        println!("No cards to study.");
        return;
    }

    // Shuffle cards
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);

    for card in cards {
        let deck_card = deck
            .cards
            .iter()
            .find(|deck_card| deck_card.id == card.id)
            .unwrap();

        let fsrs = FSRS::default();
        let schedules = fsrs.repeat(card.fsrs.clone(), Utc::now());

        Text::new(&deck_card.front).prompt().unwrap();

        let rating = match Select::new(
            format!(
                "The correct answer is: {}. How well did you do?",
                deck_card.back
            )
            .as_str(),
            Rating::iter().collect(),
        )
        .prompt()
        {
            Ok(rating) => rating.clone(),
            Err(_) => {
                println!("Failed to get rating selection.");
                return;
            }
        };

        let new_schedule = schedules[&FSRSRating::from(rating)].clone();

        // Update card in progress
        progress
            .cards
            .iter_mut()
            .find(|progress_card| progress_card.id == card.id)
            .unwrap()
            .fsrs = new_schedule.card;

        progress.save();

        let mut log = match deck.log() {
            Ok(log) => log,
            Err(err) => {
                panic!("Could not get log: {}", err);
            }
        };

        log.entries.push(LogEntry {
            last_card: card.clone(),
            log: new_schedule.review_log,
        });

        log.save();
    }
}
