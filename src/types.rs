use std::fmt;

use rs_fsrs::{Card as FSRSCard, Rating as FSRSRating};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    pub id: String,
    pub description: String,
    pub cards: Vec<Card>,
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
