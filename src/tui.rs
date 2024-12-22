use clap::Parser;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::*;

pub mod deck_select;
pub mod init_deck;

use super::run_command;

pub fn setup(siv: &mut cursive::Cursive) {
    siv.add_global_callback(':', |s| open_cmd_dialog(s));

    siv.add_fullscreen_layer(
        cursive::views::TextView::new("")
            .with_name("content")
            .scrollable()
            .full_screen(),
    );
}

pub fn open_cmd_dialog(siv: &mut cursive::Cursive) {
    siv.add_global_callback(Key::Esc, |s| {
        s.pop_layer();
        s.clear_global_callbacks(Key::Esc);
    });

    let (x, _) = termion::terminal_size().unwrap();

    let command_box = EditView::new()
        .on_submit(|s, cmd| {
            let args = crate::commands::Args::try_parse_from(
                vec!["ducki"]
                    .into_iter()
                    .chain(cmd.split_whitespace())
                    .collect::<Vec<_>>(),
            );
            s.clear();

            s.add_fullscreen_layer(
                cursive::views::TextView::new("")
                    .with_name("content")
                    .scrollable()
                    .full_screen(),
            );

            // TODO: Handle error
            match args {
                Ok(args) => run_command(args.cmd, &mut Some(s)),
                Err(_) => {}
            }
        })
        .with_name("command_box")
        .fixed_width(usize::from(x / 2));

    siv.add_layer(
        Dialog::around(LinearLayout::horizontal().child(command_box)).title("Run command"),
    );
}
