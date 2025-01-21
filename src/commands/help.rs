use clap::CommandFactory;

use super::Args;

pub fn run(siv: &mut Option<&mut cursive::Cursive>) {
    let help_text = Args::command().render_help();

    match siv {
        Some(s) => {
            s.call_on_name("content", |v: &mut cursive::views::TextView| {
                v.set_content(format!("{help_text}"));
            });
        }
        None => {
            println!("{help_text}");
        }
    }
}
