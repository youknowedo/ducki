use serde::{Deserialize, Serialize};
use std::fs;
use std::{env, fmt};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub decks: Vec<DeckEntry>,
}

impl Default for Config {
    fn default() -> Self {
        Config { decks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeckEntry {
    pub id: String,
    pub path: String,
}

impl fmt::Display for DeckEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn get_config() -> Config {
    // TODO: Use real home directory
    let home = match env::home_dir() {
        Some(home) => home,
        None => {
            panic!("Could not find home directory");
        }
    };

    match fs::read_to_string(home.join(".ducki")) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(err) => {
                panic!("Could not parse config file: {}", err);
            }
        },
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Config { decks: Vec::new() }
            } else {
                panic!("Could not read config file: {}", err);
            }
        }
    }
}

pub fn save_config(config: Config) {
    // TODO: Use real home directory
    let home = match env::home_dir() {
        Some(home) => home,
        None => {
            panic!("Could not find home directory");
        }
    };

    match fs::write(home.join(".ducki"), serde_json::to_string(&config).unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not write config file: {}", err);
        }
    }
}
