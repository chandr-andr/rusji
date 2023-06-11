use cursive::Cursive;

use crate::jira::common::{
    button::{CallbackWithButton, ClickableCallback},
    views::ButtonView,
};

use super::{enums::TaskActions, views::ChangeTransitionActionView};

#[derive(Default)]
pub struct TasksActionsButtons<'a> {
    pub buttons: Vec<CallbackWithButton<'a, TaskActions>>,
}



impl<'a> TasksActionsButtons<'a> {
    pub fn add_button(
        &mut self,
        _: &mut Cursive,
        event: char,
        name: &'a str,
        action_fn: fn(&mut Cursive),
        variant: TaskActions,
    ) {
        self.buttons
            .push(CallbackWithButton::new(variant, event, name, action_fn))
    }
}

/// Build all tasks actions buttons.
pub fn build_buttons<'a>(cursive: &mut Cursive) -> TasksActionsButtons<'a> {
    let mut buttons = TasksActionsButtons::default();

    buttons.add_button(
        cursive,
        'c',
        "Change issue status",
        |cursive: &mut Cursive| {
            let action_view = ChangeTransitionActionView::new(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::StatusChange,
    );

    buttons.add_button(
        cursive,
        'e',
        "Change issue executor",
        |cursive: &mut Cursive| {
            let action_view = ChangeTransitionActionView::new(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::ChangeExecutor,
    );

    buttons.add_button(
        cursive,
        'r',
        "Change issue release",
        |cursive: &mut Cursive| {
            let action_view = ChangeTransitionActionView::new(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::ChangeRelease,
    );

    buttons
}
