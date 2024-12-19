use cursive::View;
use cursive_hjkl::HjklToDirectionWrapperView;

macro_rules! tprint {
    ($to_console: expr, $siv: ident, $($arg:tt)*) => {
        match $to_console {
            true => {
                println!($($arg)*);
            }
            false => {

            }
        }
    };
}

pub fn hjkl<V: View>(view: V) -> HjklToDirectionWrapperView<V> {
    tprint!(true, siv, "HJKL wrapper applied\n");
    HjklToDirectionWrapperView::new(view)
}
