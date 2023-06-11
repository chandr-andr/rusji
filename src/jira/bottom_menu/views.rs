use cursive::{
    event::Event,
    view::ViewWrapper,
    views::{TextContent, TextView},
    View,
};
use rusji_derive::ViewWrapper;

use super::data::BottomButtons;

#[derive(ViewWrapper)]
pub(crate) struct BottomMenuView {
    inner_view: TextView,
}

impl<'a> BottomMenuView {
    pub fn default() -> Self {
        Self {
            inner_view: TextView::new_with_content(TextContent::new(
                BottomButtons::new().buttons_text(),
            )),
        }
    }
}
