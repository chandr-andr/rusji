use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{Dialog, NamedView, ScrollView, SelectView},
    View,
};
use rusji_derive::ViewWrapper;

use crate::{
    jira::common::views::{JiraView, ToggleableView},
    jira_data::JiraData,
};

#[derive(ViewWrapper)]
pub(crate) struct MenuView {
    inner_view: NamedView<Dialog>,
}

impl JiraView for MenuView {
    fn view_name() -> String {
        "MenuView".into()
    }

    fn get_view(
        cursive: &mut cursive::Cursive,
    ) -> cursive::views::ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    fn main_dialog_name() -> String {
        "MainViewDialogName".into()
    }

    fn get_main_dialog(&mut self) -> cursive::views::ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ToggleableView for MenuView {
    fn toggle_on_view(cursive: &mut cursive::Cursive) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();
        let mut jira_data_guard = jira_data.write().unwrap();
        jira_data_guard.activated_views.push(Self::view_name());
    }
}

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
