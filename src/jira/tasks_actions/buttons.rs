use crate::jira::common::button::CustomizableButton;

pub struct TasksActionsButtons<'a> {
    buttons: Vec<CustomizableButton<'a>>,
}

impl<'a> TasksActionsButtons<'a> {
    fn new() -> Self {
        let buttons: Vec<CustomizableButton> = Vec::default();
        Self { buttons: buttons }
    }
}
