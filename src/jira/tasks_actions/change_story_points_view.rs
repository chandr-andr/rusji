use cursive::{
    view::ViewWrapper,
    views::{Dialog, EditView, NamedView, ResizedView},
    View,
};

use rusji_derive::ViewWrapper;

use crate::jira::common::views::{
    ButtonView, JiraViewWithName, ToggleableView,
};

#[derive(ViewWrapper)]
struct ChangeSPView {
    inner_view: NamedView<ResizedView<Dialog>>,
}

impl ToggleableView for ChangeSPView {}

impl ButtonView for ChangeSPView {
    fn inner_view(self) -> NamedView<ResizedView<Dialog>> {
        self.inner_view
    }
}

impl JiraViewWithName for ChangeSPView {
    /// Returns name of the `ChangeSPView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeSPView".into()
    }

    /// Returns instance of `ChangeSPView`
    fn get_view(
        cursive: &mut cursive::Cursive,
    ) -> cursive::views::ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

#[derive(ViewWrapper)]
struct ChangeSPEditView {
    inner_view: Dialog,
}
