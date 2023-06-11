use cursive::Cursive;

use crate::jira::common::button::{Button, CustomizableButton};

pub struct TasksActionsButtons<'a> {
    buttons: Vec<CustomizableButton<'a>>,
}

impl<'a> Default for TasksActionsButtons<'a> {
    fn default() -> Self {
        Self {
            buttons: Vec::default(),
        }
    }
}

impl<'a> TasksActionsButtons<'a> {
    fn add_button(
        &mut self,
        cursive: &mut Cursive,
        event: char,
        name: &'a str,
        action_fn: fn(&mut Cursive),
    ) {
        self.buttons
            .push(CustomizableButton::new(event, name, action_fn))
    }
}
