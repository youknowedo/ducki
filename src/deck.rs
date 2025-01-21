use std::{
    fs,
    path::{Path, PathBuf},
};

use rs_fsrs::ReviewLog;
use serde::{Deserialize, Serialize};

use crate::{config::Config, progress::ProgressCard};

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Option<Config>,

    pub id: String,
    pub description: String,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn get(id: String) -> Result<Deck, String> {
        let config = match crate::config::get_config() {
            Ok(config) => config,
            Err(err) => return Err(format!("Could not get config: {}", err)),
        };

        let deck_entry = match config.decks.iter().find(|deck| deck.id == id) {
            Some(deck) => deck.clone(),
            None => return Err("Deck not found in config.".to_string()),
        };

        let deck_path = std::path::Path::new(deck_entry.path.as_str());

        let deck: Deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
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

        Ok(deck)
    }

    pub fn save(&self) -> Result<(), String> {
        let deck_path = std::path::Path::new(self.config.as_ref().unwrap().decks[0].path.as_str());

        let deck_as_string = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(err) => {
                return Err(format!("Could not serialize deck: {}", err));
            }
        };

        match fs::write(deck_path.join("deck.json"), deck_as_string) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("Could not write deck file: {}", err));
            }
        };

        Ok(())
    }

    fn logs_path(&self, deck_id: String) -> PathBuf {
        match &self.config {
            None => panic!("Deck config not set."),
            Some(config) => match config.decks.iter().find(|deck| deck.id == deck_id) {
                Some(deck) => Path::new(deck.path.as_str()).join(".logs.json"),
                None => {
                    panic!("Deck not found in config.");
                }
            },
        }
    }

    pub fn get_logs(&self, deck_id: String) -> Vec<Log> {
        let logs_path = self.logs_path(deck_id);

        match fs::read_to_string(logs_path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(config) => config,
                Err(err) => {
                    panic!("Could not parse config file: {}", err);
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Vec::new()
                } else {
                    panic!("Could not read config file: {}", err);
                }
            }
        }
    }

    pub fn save_logs(&self, deck_id: String, new_logs: Vec<Log>) {
        let logs_path = self.logs_path(deck_id);

        match fs::write(logs_path, serde_json::to_string(&new_logs).unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Could not write config file: {}", err);
            }
        }
    }

    pub fn add_log(&self, deck_id: String, log: Log) {
        let mut logs = self.get_logs(deck_id.clone());
        logs.push(log);
        self.save_logs(deck_id.clone(), logs);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            config: None,
            id: String::new(),
            description: String::new(),
            cards: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Log {
    pub last_card: ProgressCard,
    pub log: ReviewLog,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
}
