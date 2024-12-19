use clap::Parser;
use cursive::event::Key;
use cursive::{traits::*, CursiveExt};
use cursive::views::*;

use super::run_command;

pub fn run(siv: &mut cursive::Cursive) {
    siv.add_global_callback(':', |s| open_cmd_dialog(s));
    siv.add_global_callback(Key::Esc, |s| {
        s.pop_layer();
    });

    siv.run();
}

pub fn open_cmd_dialog(siv: &mut cursive::Cursive) {
    let (x, _) = termion::terminal_size().unwrap();

    let command_box = EditView::new()
        .on_submit(|s, cmd| {
            let args = crate::commands::Args::try_parse_from(
                vec!["ducki"]
                    .into_iter()
                    .chain(cmd.split_whitespace())
                    .collect::<Vec<_>>(),
            );            
            s.pop_layer();

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
