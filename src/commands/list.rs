pub fn run() {
    let config = crate::config::get_config();

    for deck in config.decks {
        println!("{}", deck.id);
    }
}
