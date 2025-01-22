use crate::deck::{
    log::LogEntry,
    progress::{Progress, ProgressCard, Rating},
    Card, Deck,
};
use chrono::Utc;
use cursive::{
    views::{Dialog, TextView},
    Cursive,
};
use rand::seq::SliceRandom;
use rs_fsrs::{Card as FSRSCard, Rating as FSRSRating, SchedulingInfo, FSRS};
use std::{collections::HashMap, fs};

use super::deck_select;

struct Error {
    kind: ErrorKind,
    message: String,
}

enum ErrorKind {
    NotFound,
    Io,
}

#[derive(Clone)]
struct StudyData {
    deck_id: String,
    progress_cards: Vec<ProgressCard>,
    deck_cards: Vec<Card>,
}

pub fn run(siv: &mut Cursive, id: String) {
    let mut deck = match setup_deck(id) {
        Ok(val) => val,
        Err(err) => {
            siv.add_layer(Dialog::info(err.message));
            return;
        }
    };

    let mut progress_cards = match setup_cards(&mut deck) {
        Ok(val) => val,
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                siv.add_layer(
                    Dialog::around(TextView::new(err.message)).button("Ok", |s| {
                        s.pop_layer();
                        deck_select::run(s);
                    }),
                );
                return;
            }
            _ => {
                siv.add_layer(Dialog::info(err.message));
                return;
            }
        },
    };

    // Shuffle cards
    let mut rng = rand::thread_rng();
    progress_cards.shuffle(&mut rng);

    let mut deck_cards = deck.cards.clone();
    deck_cards = progress_cards
        .iter()
        .map(|progress_card| {
            deck_cards
                .iter()
                .find(|deck_card| deck_card.id == progress_card.id)
                .unwrap()
                .clone()
        })
        .collect();

    let study_data = StudyData {
        deck_id: deck.id.clone(),
        progress_cards,
        deck_cards,
    };

    study(siv, study_data)
}

fn study(siv: &mut Cursive, mut study_data: StudyData) {
    let progress_card = match study_data.progress_cards.pop() {
        Some(progress_card) => progress_card,
        None => {
            siv.add_layer(Dialog::info("No more cards to study."));
            return;
        }
    };
    let deck_card = match study_data.deck_cards.pop() {
        Some(deck_card) => deck_card,
        None => {
            siv.add_layer(Dialog::info("No more cards to study."));
            return;
        }
    };

    let fsrs = FSRS::default();
    let schedules = fsrs.repeat(progress_card.fsrs.clone(), Utc::now());

    siv.add_layer(
        Dialog::around(TextView::new(deck_card.front.clone()))
            .title("Study")
            .button("Show answer", {
                move |s| {
                    s.pop_layer();

                    let progress_card = progress_card.clone();
                    let schedules = schedules.clone();

                    s.add_layer(
                        Dialog::around(TextView::new(deck_card.back.clone()))
                            .title("Study")
                            .button("Easy", {
                                let study_data = study_data.clone();
                                move |s| {
                                    s.pop_layer();

                                    update_progress(
                                        s,
                                        study_data.clone(),
                                        &progress_card,
                                        &schedules,
                                        Rating::Easy,
                                    );
                                }
                            }),
                    );
                }
            }),
    );
}

fn update_progress(
    siv: &mut Cursive,
    study_data: StudyData,
    progress_card: &ProgressCard,
    schedules: &HashMap<rs_fsrs::Rating, SchedulingInfo>,
    rating: Rating,
) {
    let mut deck = match setup_deck(study_data.deck_id.clone()) {
        Ok(val) => val,
        Err(err) => {
            siv.add_layer(Dialog::info(err.message));
            return;
        }
    };

    let deck_path = match deck.path() {
        Ok(path) => path,
        Err(err) => {
            siv.add_layer(Dialog::info(format!("Could not get deck path: {}", err)));
            return;
        }
    };
    let progress_path = deck_path.join(".progress.json");

    let new_schedule = schedules[&FSRSRating::from(rating)].clone();

    // Get progress
    let mut progress: Progress = match fs::read_to_string(progress_path.clone()) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(progress) => progress,
            Err(err) => {
                return siv.add_layer(Dialog::info(format!(
                    "Could not parse progress file: {}",
                    err
                )));
            }
        },
        Err(err) => {
            return siv.add_layer(Dialog::info(format!(
                "Could not read progress file: {}",
                err
            )));
        }
    };

    progress
        .cards
        .iter_mut()
        .find(|pc| pc.id == progress_card.id)
        .unwrap()
        .fsrs = new_schedule.card;

    let progress_json = serde_json::to_string_pretty(&progress).unwrap();
    fs::write(progress_path, progress_json).unwrap();

    let mut log = match deck.log() {
        Ok(log) => log,
        Err(err) => {
            return siv.add_layer(Dialog::info(format!("Could not get log: {}", err)));
        }
    };

    log.entries.push(LogEntry {
        last_card: progress_card.clone(),
        log: new_schedule.review_log,
    });

    study(siv, study_data.clone());
}

fn setup_deck<'a>(id: String) -> Result<Deck, Error> {
    let deck = match Deck::get(id) {
        Ok(deck) => deck,
        Err(err) => {
            return Err(Error {
                kind: ErrorKind::Io,
                message: format!("Could not get deck: {}", err),
            });
        }
    };

    Ok(deck)
}

fn setup_cards(deck: &mut Deck) -> Result<Vec<ProgressCard>, Error> {
    let now = Utc::now();

    let deck_path = match deck.path() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error {
                kind: ErrorKind::Io,
                message: format!("Could not get deck path: {}", err),
            });
        }
    };
    let progress_path = deck_path.join(".progress.json");

    let mut progress = match deck.progress() {
        Ok(progress) => progress,
        Err(err) => {
            return Err(Error {
                kind: ErrorKind::Io,
                message: format!("Could not get progress: {}", err),
            });
        }
    };

    // If the progress file does not exist, create a new one
    if !progress_path.exists() {
        progress = Progress::new(deck.id.clone());
    }

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
        return Err(Error {
            kind: ErrorKind::NotFound,
            message: "No cards to study.".to_string(),
        });
    }

    Ok(cards)
}
