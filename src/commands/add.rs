use std::env::current_dir;

use clap::Parser;
use cursive::views::Dialog;
use inquire::Confirm;

#[derive(Parser, Debug, Clone)]
pub struct AddArgs {
    id: Option<String>,
}

pub fn run(args: AddArgs, siv: &mut Option<&mut cursive::Cursive>) {
    match siv {
        Some(s) => s.add_layer(Dialog::info("This command is not available in the TUI.")),
        None => {
            match siv {
                Some(s) => s.quit(),
                None => {}
            }
            terminal(args)
        }
    }
}

fn terminal(args: AddArgs) {
    let mut config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    let current_basename = match current_dir() {
        Ok(dir) => dir
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string(),
        Err(err) => panic!("Could not get current directory: {}", err),
    };

    let (id, path) = match args.id {
        Some(id) => {
            if id.as_str() == (".") {
                (current_basename, current_dir().unwrap())
            } else {
                (id.clone(), current_dir().unwrap().join(&id))
            }
        }
        None => (current_basename, current_dir().unwrap()),
    };

    if config.decks.iter().find(|deck| deck.id == id).is_some() {
        if let Ok(overwrite) = Confirm::new("Deck already exists in config. Overwrite?")
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

    config.decks.push(crate::config::DeckEntry {
        id,
        path: path.to_str().unwrap().to_string(),
    });
    crate::config::save_config(config);
}
