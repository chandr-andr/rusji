use cursive::{
    view::{Finder, ViewWrapper},
    views::{Dialog, NamedView},
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::jira::common::views::JiraView;

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

impl MenuView {
    // fn new(cursive: &mut Cursive) -> Self {
    //     let SelectView::<String>::new()
    // }
}
