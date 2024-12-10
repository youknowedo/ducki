use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use rs_fsrs::{Card as FSRSCard, Rating as FSRSRating, ReviewLog};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck<'a> {
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Option<&'a Config>,

    pub id: String,
    pub description: String,
    pub cards: Vec<Card>,
}

impl Deck<'_> {
    fn logs_path(&self, deck_id: String) -> PathBuf {
        match self.config {
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

impl Default for Deck<'_> {
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
    pub last_card: Card,
    pub log: ReviewLog,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Progress {
    pub cards: Vec<ProgressCard>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgressCard {
    pub id: String,
    pub fsrs: FSRSCard,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Eq)]
pub enum Rating {
    Easy,
    Good,
    Hard,
    Again,
}
impl Rating {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static VARIANTS: [Rating; 4] = [Rating::Easy, Rating::Good, Rating::Hard, Rating::Again];
        VARIANTS.iter()
    }
}
impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rating_str = match self {
            Rating::Easy => "Easy",
            Rating::Good => "Good",
            Rating::Hard => "Hard",
            Rating::Again => "Again",
        };
        write!(f, "{}", rating_str)
    }
}
impl From<Rating> for FSRSRating {
    fn from(item: Rating) -> FSRSRating {
        match item {
            Rating::Easy => FSRSRating::Easy,
            Rating::Good => FSRSRating::Good,
            Rating::Hard => FSRSRating::Hard,
            Rating::Again => FSRSRating::Again,
        }
    }
}
