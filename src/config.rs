use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::result::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub decks: Vec<DeckEntry>,
}
impl Config {
    pub fn get() -> Result<Config, String> {
        let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
            Some(dirs) => dirs,
            None => {
                return Err("Could not find project directories".to_owned());
            }
        };

        match fs::exists(project_dirs.config_dir().join("config.json")) {
            Ok(exists) => {
                if !exists {
                    return Ok(Config { decks: Vec::new() });
                }
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Ok(Config { decks: Vec::new() });
                } else {
                    panic!("Could not check if config file exists: {}", err);
                }
            }
        }

        match fs::read_to_string(project_dirs.config_dir().join("config.json")) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(config) => Ok(config),
                Err(err) => {
                    return Err(format!("Could not deserialize config: {}", err));
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Ok(Config { decks: Vec::new() })
                } else {
                    return Err("Could not read config file".to_owned());
                }
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
            Some(dirs) => dirs,
            None => {
                return Err("Could not find project directories".to_owned());
            }
        };
        let config_path = project_dirs.config_dir().join("config.json");

        let config_as_string = match serde_json::to_string(&self) {
            Ok(contents) => contents,
            Err(err) => {
                return Err(format!("Could not serialize config: {}", err));
            }
        };

        match fs::exists(config_path.clone()) {
            Ok(exists) => {
                if !exists {
                    match fs::create_dir_all(project_dirs.config_dir()) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(format!("Could not create config directory: {}", err));
                        }
                    }
                }
            }
            Err(err) => {
                return Err(format!("Could not check if config file exists: {}", err));
            }
        }

        match fs::write(config_path, config_as_string) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not write config file: {}", err)),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Config { decks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeckEntry {
    pub id: String,
    pub path: PathBuf,
}
impl DeckEntry {
    pub fn new(id: String, path: PathBuf) -> DeckEntry {
        DeckEntry { id, path }
    }
}

impl fmt::Display for DeckEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
