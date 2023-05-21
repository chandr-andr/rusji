use cursive::{
    view::ViewWrapper,
    views::{Dialog, TextView},
    Cursive,
};

pub(crate) struct TryAgainView {
    inner_view: Dialog,
}

impl ViewWrapper for TryAgainView {
    type V = Dialog;

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

pub(crate) struct FailedAttemptView {
    inner_view: Dialog,
}

impl ViewWrapper for FailedAttemptView {
    type V = Dialog;

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
