use cursive::CursiveExt;

use crate::tui;

pub fn run(siv: &mut cursive::Cursive) {
    tui::deck_select::run(siv);

    siv.run();
}
