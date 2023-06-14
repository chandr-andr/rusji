use std::str::FromStr;

use cursive::Cursive;

use crate::jira::common::{
    buttons_variants::ButtonVariant, views::ButtonView,
};

use super::{
    change_assignee_views::ChangeAssigneeSearchView,
    change_transition_view::ChangeTransitionActionView,
};

#[derive(Clone, Copy)] // TODO: remove Clone, Copy
pub enum TaskActions {
    StatusChange,
    ChangeAssignee,
    ChangeRelease,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TaskActionParseError;

impl FromStr for TaskActions {
    type Err = TaskActionParseError;
    fn from_str(str_action: &str) -> Result<Self, Self::Err> {
        match str_action {
            "Change status" => Ok(TaskActions::StatusChange),
            "Change executor" => Ok(TaskActions::ChangeAssignee),
            "Change release" => Ok(TaskActions::ChangeRelease),
            _ => Err(TaskActionParseError {}),
        }
    }
}

impl<'a> From<TaskActions> for &'a str {
    fn from(action: TaskActions) -> Self {
        match action {
            TaskActions::StatusChange => "Change status",
            TaskActions::ChangeAssignee => "Change executor",
            TaskActions::ChangeRelease => "Change release",
        }
    }
}

impl<'a> ButtonVariant<'a> for TaskActions {}

impl TaskActions {
    /// Returns all available actions.
    pub fn get_actions() -> Vec<&'static str> {
        vec![
            Self::StatusChange.into(),
            Self::ChangeAssignee.into(),
            Self::ChangeRelease.into(),
        ]
    }

    /// Returns new action view based on `TaskActions` enum.
    pub fn get_view(self, cursive: &mut Cursive) -> impl ButtonView {
        match self {
            TaskActions::StatusChange => {
                ChangeTransitionActionView::new(cursive)
            }
            TaskActions::ChangeAssignee => {
                ChangeTransitionActionView::new(cursive)
            }
            TaskActions::ChangeRelease => {
                ChangeTransitionActionView::new(cursive)
            }
        }
    }
}
