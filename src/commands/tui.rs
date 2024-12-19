use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};

pub fn run() {
    let mut siv = Cursive::new();

    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}
