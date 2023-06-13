use cursive::Cursive;

use crate::jira::common::button::{CallbackWithButton, ClickableCallback};

use super::enums::TaskActions;

#[derive(Default)]
pub struct TasksActionsButtons<'a> {
    pub buttons: Vec<CallbackWithButton<'a, TaskActions>>,
}

/// Methods for actions buttons.
impl<'a> TasksActionsButtons<'a> {
    /// Add new button to the structure.
    ///
    /// TODO: Make possible to configure event param
    /// from configuration file.
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
            let action_view = TaskActions::StatusChange.get_view(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::StatusChange,
    );

    buttons.add_button(
        cursive,
        'e',
        "Change issue executor",
        |cursive: &mut Cursive| {
            let action_view = TaskActions::StatusChange.get_view(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::ChangeExecutor,
    );

    buttons.add_button(
        cursive,
        'r',
        "Change issue release",
        |cursive: &mut Cursive| {
            let action_view = TaskActions::StatusChange.get_view(cursive);
            cursive.add_layer(action_view);
        },
        TaskActions::ChangeRelease,
    );

    buttons
}
