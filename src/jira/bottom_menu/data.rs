use cursive::Cursive;

use crate::jira::{
    common::button::{Button, CustomizableButton},
    menu::views::MenuView,
};

use super::helpers::build_tasks_action_view;

pub struct BottomButtons<'a> {
    pub buttons: Vec<CustomizableButton<'a>>,
}

impl<'a> BottomButtons<'a> {
    pub fn new() -> Self {
        let mut buttons: Vec<CustomizableButton> = Vec::default();
        buttons.push(CustomizableButton::new(
            'a',
            "task actions",
            build_tasks_action_view,
        ));
        buttons.push(CustomizableButton::new(
            'm',
            "menu",
            |cursive: &mut Cursive| {
                let menu = MenuView::new(cursive);
                cursive.add_layer(menu);
            },
        ));
        buttons.push(CustomizableButton::new(
            'q',
            "quit",
            |cursive: &mut Cursive| {
                cursive.quit();
            },
        ));

        Self { buttons }
    }

    pub fn buttons_text(&self) -> String {
        self.buttons
            .iter()
            .map(|button| format!("| {} |", button.full_name()))
            .collect()
    }
}
