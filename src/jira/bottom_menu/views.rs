use cursive::{
    view::{Nameable, ViewWrapper},
    views::{Dialog, NamedView, TextContent, TextView},
    View,
};

pub(crate) struct BottomMenuView {
    inner_view: NamedView<Dialog>,
}

impl ViewWrapper for BottomMenuView {
    type V = NamedView<Dialog>;

    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        Some(f(&mut self.inner_view))
    }

    fn wrap_call_on_any<'a>(
        &mut self,
        selector: &cursive::view::Selector<'_>,
        callback: cursive::event::AnyCb<'a>,
    ) {
        self.with_view_mut(|v| v.call_on_any(selector, callback));
    }
}

impl BottomMenuView {
    fn default() -> Self {
        Self {
            inner_view: Dialog::new()
                .content(TextView::new_with_content(TextContent::new(
                    "m - menu, x - exit",
                )))
                .with_name("Base"),
        }
    }
}
