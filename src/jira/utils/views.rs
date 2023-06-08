use cursive::{view::ViewWrapper, views::Dialog, Cursive, View};
use rusji_derive::ViewWrapper;

#[derive(ViewWrapper)]
pub(crate) struct TryAgainView {
    inner_view: Dialog,
}

#[derive(ViewWrapper)]
pub(crate) struct FailedAttemptView {
    inner_view: Dialog,
}

impl FailedAttemptView {
    pub fn new(error_text: &str) -> Self {
        Self {
            inner_view: Dialog::new().title(error_text).button(
                "Exit",
                |cursive: &mut Cursive| {
                    cursive.pop_layer();
                },
            ),
        }
    }
}
