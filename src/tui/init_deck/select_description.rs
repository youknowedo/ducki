use super::save_deck::save_deck;
use super::InitData;
use crate::util::{read_temp_file_with_siv, write_temp_file_with_siv};
use cursive::view::Resizable;
use cursive::views::{Dialog, EditView};

pub fn select_description(siv: &mut cursive::Cursive, temp_file_id: String) {
    siv.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(move |s, description| {
                    s.pop_layer();
                    let mut data = match read_temp_file_with_siv::<InitData>(s, &temp_file_id) {
                        Ok(data) => data,
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                            return;
                        }
                    };

                    data.deck.description = description.to_string();

                    match write_temp_file_with_siv(s, &temp_file_id, &data) {
                        Ok(_) => save_deck(s, temp_file_id.clone()),
                        Err(err) => {
                            s.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                        }
                    }
                })
                .fixed_width(50),
        )
        .title("Select description"),
    );
}
