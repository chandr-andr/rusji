use cursive::{
    view::{Nameable, ViewWrapper},
    views::{Dialog, NamedView, TextContent, TextView},
    View,
};
use rusji_derive::ViewWrapper;

#[derive(ViewWrapper)]
pub(crate) struct MenuView {
    inner_view: NamedView<Dialog>,
}

impl MenuView {
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
