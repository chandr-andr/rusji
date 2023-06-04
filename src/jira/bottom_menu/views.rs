use cursive::{
    view::{Nameable, ViewWrapper},
    views::{Dialog, NamedView, TextContent, TextView},
    View,
};
use rusji_derive::ViewWrapper;

use super::data::Buttons;

#[derive(ViewWrapper)]
pub(crate) struct BottomMenuView {
    inner_view: NamedView<Dialog>,
}

impl BottomMenuView {
    fn default() -> Self {
        let buttons = Buttons::new();
        Self {
            inner_view: Dialog::new()
                .content(TextView::new_with_content(TextContent::new(
                    "m - menu, x - exit",
                )))
                .with_name("Base"),
        }
    }
}
