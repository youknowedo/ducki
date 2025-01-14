use cursive::views::Dialog;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::result::Result;

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

pub fn get_config() -> Result<Config, String> {
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

pub fn save_config(config: Config) {
    _save_config(config, None);
}
pub fn save_config_with_siv(siv: &mut cursive::Cursive, config: Config) {
    _save_config(config, Some(siv));
}

fn _save_config(config: Config, siv: Option<&mut cursive::Cursive>) {
    let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
        Some(dirs) => dirs,
        None => {
            match siv {
                Some(siv) => siv.add_layer(Dialog::info("Could not find project directories")),
                None => {
                    panic!("Could not find project directories");
                }
            }
            return;
        }
    };
    let config_path = project_dirs.config_dir().join("config.json");

    let config_as_string = match serde_json::to_string(&config) {
        Ok(contents) => contents,
        Err(err) => {
            match siv {
                Some(siv) => siv.add_layer(Dialog::info("Could not serialize config")),
                None => {
                    panic!("Could not serialize config: {}", err);
                }
            };
            return;
        }
    };

    match fs::exists(config_path.clone()) {
        Ok(exists) => {
            if !exists {
                match fs::create_dir_all(project_dirs.config_dir()) {
                    Ok(_) => {}
                    Err(err) => {
                        match siv {
                            Some(siv) => siv.add_layer(Dialog::info(format!(
                                "Could not create config directory: {}",
                                err
                            ))),
                            None => {
                                panic!("Could not create config directory: {}", err);
                            }
                        };
                        return;
                    }
                }
            }
        }
        Err(err) => {
            match siv {
                Some(siv) => siv.add_layer(Dialog::info(format!(
                    "Could not check if config file exists: {}",
                    err
                ))),
                None => {
                    panic!("Could not check if config file exists: {}", err);
                }
            };
            return;
        }
    }

    match fs::write(config_path, config_as_string) {
        Ok(_) => {}
        Err(err) => {
            match siv {
                Some(siv) => siv.add_layer(Dialog::info(format!(
                    "Could not write config file: {}",
                    err
                ))),
                None => {
                    panic!("Could not write config file: {}", err);
                }
            };
            return;
        }
    }
}
