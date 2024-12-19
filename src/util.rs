use cursive::View;
use cursive_hjkl::HjklToDirectionWrapperView;

pub fn hjkl<V: View>(view: V) -> HjklToDirectionWrapperView<V> {
    HjklToDirectionWrapperView::new(view)
}
