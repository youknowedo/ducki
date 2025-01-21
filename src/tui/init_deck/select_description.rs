use super::save_deck;
use super::InitData;
use cursive::view::Resizable;
use cursive::views::{Dialog, EditView};

pub fn run(siv: &mut cursive::Cursive, data: InitData) {
    siv.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(move |s, description| {
                    s.pop_layer();
                    let mut data = data.clone();

                    data.deck.description = description.to_string();
                    
                    save_deck::run(s, data);
                })
                .fixed_width(50),
        )
        .title("Select description"),
    );
}
