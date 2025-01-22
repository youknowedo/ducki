use std::{fs, path::PathBuf};

use rs_fsrs::ReviewLog;
use serde::{Deserialize, Serialize};

use super::{progress::ProgressCard, Deck};

#[derive(Serialize, Deserialize, Clone)]
pub struct Log {
    #[serde(skip_serializing, skip_deserializing)]
    _path: Option<PathBuf>,

    pub deck_id: String,
    pub entries: Vec<LogEntry>,
}
impl Log {
    pub fn new(deck_id: String) -> Self {
        Log {
            _path: None,
            deck_id,
            entries: Vec::new(),
        }
    }

    pub fn get(deck_id: String) -> Result<Self, String> {
        let mut log = Log::new(deck_id);

        let deck_path = match log.path() {
            Ok(path) => path,
            Err(err) => {
                return Err(format!("Could not get log path: {}", err));
            }
        };

        let log_path = deck_path.join(".log.json");

        log = match fs::read_to_string(log_path) {
            Ok(contents) => match serde_json::from_str::<Log>(&contents) {
                Ok(log) => log,
                Err(err) => {
                    return Err(format!("Could not parse log file: {}", err));
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err("Log file not found.".to_string());
                } else {
                    return Err(format!("Could not read log file: {}", err));
                }
            }
        };

        Ok(log)
    }

    pub fn save(&mut self) {
        let deck_path = match self.path() {
            Ok(path) => path,
            Err(err) => {
                panic!("Could not get deck path: {}", err);
            }
        };

        let log_path = deck_path.join(".log.json");

        match fs::write(log_path, serde_json::to_string(&self).unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Could not write config file: {}", err);
            }
        }
    }

    fn path(&self) -> Result<PathBuf, String> {
        let mut deck = match Deck::get(self.deck_id.clone()) {
            Ok(deck) => deck,
            Err(err) => {
                return Err(format!("Could not get deck: {}", err));
            }
        };
        let deck_path = match deck.path() {
            Ok(path) => path,
            Err(err) => {
                return Err(format!("Could not get deck path: {}", err));
            }
        };

        Ok(deck_path.join(".progress.json"))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub last_card: ProgressCard,
    pub log: ReviewLog,
}
