use cursive::{
    view::ViewWrapper,
    views::{Dialog, TextView},
    Cursive, View,
};
use rusji_derive::ViewWrapper;

#[derive(ViewWrapper)]
pub(crate) struct TryAgainView {
    inner_view: Dialog,
}

impl TryAgainView {
    pub fn new<T>(error_text: &str, try_again_fn: T) -> Self
    where
        T: 'static + Fn(&mut Cursive),
    {
        Self {
            inner_view: Dialog::new()
                .title("Connection error!")
                .content(TextView::new(error_text))
                .button("Try again", try_again_fn),
        }
    }
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
