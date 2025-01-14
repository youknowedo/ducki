pub fn run(siv: &mut Option<&mut cursive::Cursive>) {
    let config = match crate::config::get_config() {
        Ok(config) => config,
        Err(err) => panic!("Could not get config: {}", err),
    };

    match siv {
        Some(s) => {
            s.call_on_name("content", |v: &mut cursive::views::TextView| {
                v.set_content("");
            });
        }
        None => {}
    }

    for deck in config.decks {
        match siv {
            Some(s) => {
                s.call_on_name("content", |v: &mut cursive::views::TextView| {
                    v.append(format!("{}\n", deck.id));
                });
            }
            None => {
                println!("{}", deck.id);
            }
        }
    }
}
