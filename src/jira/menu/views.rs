use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{Dialog, NamedView, ScrollView, SelectView},
    View,
};
use rusji_derive::ViewWrapper;

use crate::jira::common::views::{
    JiraViewWithName, JiraWithDialogView, ToggleableView,
};

#[derive(ViewWrapper)]
pub(crate) struct MenuView {
    inner_view: NamedView<Dialog>,
}

impl JiraViewWithName for MenuView {
    fn view_name() -> String {
        "MenuView".into()
    }

    fn get_view(
        cursive: &mut cursive::Cursive,
    ) -> cursive::views::ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for MenuView {
    fn main_dialog_name() -> String {
        "MainViewDialogName".into()
    }

    fn get_main_dialog(&mut self) -> cursive::views::ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ToggleableView for MenuView {}

impl MenuView {
    pub fn new(cursive: &mut cursive::Cursive) -> Self {
        Self::toggle_on_view(cursive);
        let inner_select_view = SelectView::<String>::new();

        Self {
            inner_view: Dialog::new()
                .title("Menu")
                .content(ScrollView::new(inner_select_view))
                .with_name(Self::main_dialog_name()),
        }
    }
}
