use cursive::Cursive;

use crate::jira::{common::button::Button, menu::views::MenuView};

use super::helpers::build_tasks_action_view;

pub struct BottomButtons<'a> {
    pub buttons: Vec<Button<'a>>,
}

impl<'a> BottomButtons<'a> {
    pub fn new() -> Self {
        let mut buttons: Vec<Button> = Vec::default();
        buttons.push(Button::new(
            'a',
            "task actions",
            build_tasks_action_view,
        ));
        buttons.push(Button::new('m', "menu", |cursive: &mut Cursive| {
            let menu = MenuView::new(cursive);
            cursive.add_layer(menu);
        }));
        buttons.push(Button::new('q', "quit", |cursive: &mut Cursive| {
            cursive.quit();
        }));

        Self { buttons }
    }

    pub fn buttons_text(&self) -> String {
        self.buttons
            .iter()
            .map(|button| format!("| {} |", button.full_name()))
            .collect()
    }
}
