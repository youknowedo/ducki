use clap::Parser;
use cursive::event::Key;
use cursive::views::*;
use cursive::{traits::*, CursiveExt};
use cursive_hjkl::HjklToDirectionWrapperView;

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
            s.pop_layer();

            let args = crate::commands::Args::parse_from(cmd.split_whitespace());

            crate::commands::run_command(args.cmd, s);
        })
        .with_name("command_box")
        .fixed_width(usize::from(x / 2));

    siv.add_layer(
        Dialog::around(LinearLayout::horizontal().child(command_box)).title("Run command"),
    );
}
