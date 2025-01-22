use std::{fs, path::PathBuf};

use log::Log;
use progress::Progress;
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub mod log;
pub mod progress;

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    #[serde(skip_serializing, skip_deserializing)]
    _config: Option<Config>,
    #[serde(skip_serializing, skip_deserializing)]
    _path: Option<PathBuf>,

    pub id: String,
    pub description: String,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(id: String, description: String) -> Self {
        Deck {
            _config: None,
            _path: None,
            id,
            description,
            cards: Vec::new(),
        }
    }

    pub fn get(id: String) -> Result<Self, String> {
        let mut deck = Deck::default();
        deck.id = id;

        let deck_path = match deck.path() {
            Ok(path) => path,
            Err(err) => {
                return Err(format!("Could not get deck path: {}", err));
            }
        };

        deck = match std::fs::read_to_string(deck_path.join("deck.json")) {
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

    pub fn save(&mut self) -> Result<(), String> {
        let config = match self.config() {
            Ok(config) => config,
            Err(err) => {
                return Err(err);
            }
        };

        let deck_path = std::path::Path::new(config.decks[0].path.as_str());

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

    fn config(&mut self) -> Result<Config, String> {
        match self._config {
            Some(ref config) => Ok(config.clone()),
            None => {
                let config = match Config::get() {
                    Ok(config) => config,
                    Err(err) => return Err(format!("Could not get config: {}", err)),
                };

                self._config = Some(config.clone());
                return Ok(self._config.clone().unwrap());
            }
        }
    }

    pub fn path(&mut self) -> Result<PathBuf, String> {
        match self._path {
            Some(ref path) => Ok(path.clone()),
            None => {
                let config = match self.config() {
                    Ok(config) => config,
                    Err(err) => return Err(err),
                };

                let deck_entry = match config.decks.iter().find(|deck| deck.id == self.id) {
                    Some(deck) => deck.clone(),
                    None => return Err("Deck not found in config.".to_string()),
                };

                let deck_path = std::path::Path::new(deck_entry.path.as_str());

                self._path = Some(deck_path.to_path_buf());
                return Ok(self._path.clone().unwrap());
            }
        }
    }

    pub fn progress(&mut self) -> Result<Progress, String> {
        Progress::get(self.id.clone())
    }
    pub fn log(&mut self) -> Result<Log, String> {
        Log::get(self.id.clone())
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            _config: None,
            _path: None,
            id: String::new(),
            description: String::new(),
            cards: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
}
