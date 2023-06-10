use crate::jira::common::button::Button;

pub struct TasksActionsButtons<'a> {
    buttons: Vec<Button<'a>>,
}

impl<'a> TasksActionsButtons<'a> {
    fn new() -> Self {
        let buttons: Vec<Button> = Vec::default();
        Self { buttons: buttons }
    }
}
