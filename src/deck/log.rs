use std::{fs, path::PathBuf};

use rs_fsrs::ReviewLog;
use serde::{Deserialize, Serialize};

use super::{progress::ProgressCard, Deck};

#[derive(Serialize, Deserialize, Clone)]
pub struct Log {
    #[serde(skip_serializing, skip_deserializing)]
    path: PathBuf,

    pub deck_id: String,
    pub entries: Vec<LogEntry>,
}
impl Log {
    pub fn new(deck_id: String, deck_path: PathBuf) -> Self {
        Log {
            path: deck_path.join(".log.json"),
            deck_id,
            entries: Vec::new(),
        }
    }

    pub fn get(deck: &Deck) -> Result<Self, String> {
        let log_path = deck.path.join(".log.json");

        let mut log = Log::new(deck.id.clone(), log_path.clone());

        log = match fs::read_to_string(log_path) {
            Ok(contents) => match serde_json::from_str::<Log>(&contents) {
                Ok(log) => log,
                Err(err) => {
                    return Err(format!("Could not parse log file: {}", err));
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Ok(log);
                } else {
                    return Err(format!("Could not read log file: {}", err));
                }
            }
        };

        Ok(log)
    }

    pub fn save(&mut self) {
        match fs::write(self.path.clone(), serde_json::to_string(&self).unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Could not write config file: {}", err);
            }
        }
    }
}
impl Default for Log {
    fn default() -> Self {
        Log {
            path: PathBuf::new(),
            deck_id: String::new(),
            entries: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub last_card: ProgressCard,
    pub log: ReviewLog,
}
