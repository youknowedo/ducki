use std::{fmt, fs, path::PathBuf};

use rs_fsrs::{Card as FSRSCard, Rating as FSRSRating};
use serde::{Deserialize, Serialize};

use crate::deck::Deck;

#[derive(Serialize, Deserialize, Clone)]
pub struct Progress {
    #[serde(skip_serializing, skip_deserializing)]
    _path: Option<PathBuf>,

    pub deck_id: String,
    pub cards: Vec<ProgressCard>,
}
impl Progress {
    pub fn new(deck_id: String) -> Self {
        Progress {
            _path: None,
            deck_id,
            cards: Vec::new(),
        }
    }

    pub fn get(deck_id: String) -> Result<Self, String> {
        let mut progress = Progress::new(deck_id);

        let progress_path = match progress.path() {
            Ok(path) => path,
            Err(err) => {
                return Err(format!("Could not get progress path: {}", err));
            }
        };

        progress = match fs::read_to_string(progress_path) {
            Ok(contents) => match serde_json::from_str::<Progress>(&contents) {
                Ok(progress) => progress,
                Err(err) => {
                    return Err(format!("Could not parse progress file: {}", err));
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err("Progress file not found.".to_string());
                } else {
                    return Err(format!("Could not read progress file: {}", err));
                }
            }
        };

        Ok(progress)
    }

    pub fn save(&mut self) {
        let deck_path = match self.path() {
            Ok(path) => path,
            Err(err) => {
                panic!("Could not get deck path: {}", err);
            }
        };

        let progress_path = deck_path.join(".progress.json");

        match fs::write(progress_path, serde_json::to_string(&self).unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Could not write progress file: {}", err);
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
