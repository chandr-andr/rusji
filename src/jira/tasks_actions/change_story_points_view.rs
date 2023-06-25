use cursive::{
    view::{Nameable, Resizable, ViewWrapper},
    views::{Dialog, EditView, NamedView, ResizedView},
    Cursive, View,
};

use rusji_derive::ViewWrapper;

use crate::jira::{
    common::views::{ButtonView, JiraViewWithName, ToggleableView},
    utils::helpers::calculate_view_size,
};

/// Main view for changing story points.
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

impl ChangeSPView {
    fn new(cursive: &mut Cursive) -> Self {
        let chnage_sp_view = Dialog::new()
            .title("Change story points")
            .fixed_size(calculate_view_size(cursive, 3, 7))
            .with_name(Self::view_name());
        Self {
            inner_view: chnage_sp_view,
        }
    }
}

#[derive(ViewWrapper)]
struct ChangeSPEditView {
    inner_view: Dialog,
}
