use cursive::{
    event::{Event, Key},
    Cursive,
};

use crate::jira::{
    common::button::{CallbackText, ShowOnlyCallback, StaticCallback},
    menu::views::MenuView,
};

use super::helpers::build_tasks_action_view;

pub struct BottomButtons<'a> {
    pub buttons: Vec<StaticCallback<'a, Event>>,
}

impl<'a> BottomButtons<'a> {
    pub fn new() -> Self {
        let mut buttons: Vec<StaticCallback<'a, Event>> = Vec::default();
        buttons.push(StaticCallback::new(
            Event::Char('a'),
            "a - task actions",
            build_tasks_action_view,
        ));
        buttons.push(StaticCallback::new(
            Event::Char('m'),
            "m - menu",
            |cursive: &mut Cursive| {
                let menu = MenuView::new(cursive);
                cursive.add_layer(menu);
            },
        ));
        buttons.push(StaticCallback::new(
            Event::Char('q'),
            "q - quit",
            |cursive: &mut Cursive| {
                cursive.quit();
            },
        ));

        Self { buttons }
    }

    pub fn buttons_text(&self) -> String {
        self.buttons
            .iter()
            .map(|button| format!("| {} |", button.display_text()))
            .collect()
    }
}
