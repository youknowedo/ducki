use std::{fs, path::PathBuf};

use log::Log;
use progress::Progress;
use serde::{Deserialize, Serialize};

use crate::config::{Config, DeckEntry};

pub mod log;
pub mod progress;

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    pub id: String,
    pub description: String,
    pub cards: Vec<Card>,

    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
    #[serde(skip_serializing, skip_deserializing)]
    pub progress: Progress,
    #[serde(skip_serializing, skip_deserializing)]
    pub log: Log,
}

impl Deck {
    pub fn new(id: String, path: PathBuf, description: String) -> Self {
        Deck {
            path: path.clone(),
            id: id.clone(),
            description,
            cards: Vec::new(),
            progress: Progress::new(id.clone(), path.clone()),
            log: Log::new(id.clone(), path.clone()),
        }
    }

    pub fn get(id: String) -> Result<Self, String> {
        let config = match Config::get() {
            Ok(config) => config,
            Err(err) => {
                return Err(err);
            }
        };

        let deck_entry = match config.decks.iter().find(|deck| deck.id == id) {
            Some(deck) => deck.clone(),
            None => {
                return Err("Deck not found in config.".to_string());
            }
        };
        let deck_path = deck_entry.path;

        let mut deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
            Ok(contents) => match serde_json::from_str::<Deck>(&contents) {
                Ok(deck) => deck,
                Err(err) => {
                    return Err(format!("Could not parse deck file: {}", err));
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err("Deck file not found.".to_string());
                } else {
                    return Err(format!("Could not read deck file: {}", err));
                }
            }
        };
        deck.path = deck_path;
        deck.progress = match Progress::get(&deck) {
            Ok(progress) => progress,
            Err(err) => {
                return Err(err);
            }
        };
        deck.log = match Log::get(&deck) {
            Ok(log) => log,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(deck)
    }

    pub fn save(&self) -> Result<(), String> {
        let deck_as_string = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(err) => {
                return Err(format!("Could not serialize deck: {}", err));
            }
        };

        match fs::write(self.path.join("deck.json"), deck_as_string) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!(
                    "Could not write deck file to {}: {}",
                    self.path.to_str().unwrap(),
                    err
                ));
            }
        };

        Ok(())
    }

    pub fn progress(&self) -> Result<Progress, String> {
        Progress::get(self)
    }
    pub fn log(&self) -> Result<Log, String> {
        Log::get(self)
    }

    pub fn to_entry(&self) -> DeckEntry {
        DeckEntry::new(self.id.clone(), self.path.clone())
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            path: PathBuf::new(),
            id: String::new(),
            description: String::new(),
            cards: Vec::new(),
            progress: Progress::new(String::new(), PathBuf::new()),
            log: Log::new(String::new(), PathBuf::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
}
