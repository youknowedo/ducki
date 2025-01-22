use std::{env::current_dir, fs::create_dir_all};

use clap::Parser;
use inquire::{Confirm, Text};

use crate::{config::Config, deck::Deck, tui::init_deck::run as tui};

#[derive(Parser, Debug, Clone)]
pub struct InitArgs {
    id: Option<String>,
}

pub fn run(args: InitArgs, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => tui(s, args.id),
        None => {
            match siv {
                Some(s) => s.quit(),
                None => {}
            }
            terminal(args)
        }
    }
}

fn terminal(args: InitArgs) {
    let mut config = match Config::get() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    println!("{:?}", serde_json::to_string_pretty(&config.clone()));

    let mut id = match args.id {
        Some(id) => id,
        None => match Text::new("What ID should the new deck have?")
            .with_default(".")
            .prompt()
        {
            Ok(name) => name,
            Err(err) => panic!("Could not read ID: {}", err),
        },
    };

    let path = match id.chars().next().unwrap() {
        '.' => match current_dir() {
            Ok(path) => {
                id = path
                    .components()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                path
            }
            Err(err) => panic!("Could not get current directory: {}", err),
        },
        _ => current_dir().unwrap().join(&id),
    };

    if let Err(err) = create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("Could not create directory: {}", err);
        }
    }

    if path.join("deck.json").exists() {
        if let Ok(overwrite) = Confirm::new("Deck already exists at location. Overwrite?")
            .with_default(false)
            .prompt()
        {
            if !overwrite {
                return;
            }
        } else {
            panic!("Failed to get confirmation for overwriting the deck.");
        }
    }

    if config.decks.iter().find(|deck| deck.id == id).is_some() {
        if let Ok(overwrite) = Confirm::new("Deck already exists in config. Overwrite?")
            .with_default(false)
            .prompt()
        {
            if !overwrite {
                return;
            }
            config.decks.retain(|deck| deck.id != id);
        } else {
            panic!("Failed to get confirmation for overwriting the deck.");
        }
    }

    let mut deck = Deck::new(
        id.clone(),
        match Text::new("What description should the new deck have?").prompt() {
            Ok(description) => description,
            Err(err) => panic!("Could not read description: {}", err),
        },
    );

    match deck.save() {
        Ok(_) => {}
        Err(err) => panic!("Could not save deck: {}", err),
    };

    let deck_entry = crate::config::DeckEntry {
        id,
        path: path.to_str().unwrap().to_string(),
    };

    config.decks.push(deck_entry);

    match config.save() {
        Ok(_) => {}
        Err(err) => panic!("Could not save config: {}", err),
    };
}
