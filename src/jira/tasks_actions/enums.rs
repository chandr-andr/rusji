use std::str::FromStr;

use cursive::Cursive;

use crate::jira::common::views::ActionView;

use super::views::ChangeStatusActionView;

pub enum TaskActions {
    StatusChange,
    ChangeExecutor,
    ChangeRelease,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TaskActionParseError;

impl FromStr for TaskActions {
    type Err = TaskActionParseError;
    fn from_str(str_action: &str) -> Result<Self, Self::Err> {
        match str_action {
            "Change status" => Ok(TaskActions::StatusChange),
            "Change executor" => Ok(TaskActions::ChangeExecutor),
            "Change release" => Ok(TaskActions::ChangeRelease),
            _ => Err(TaskActionParseError {}),
        }
    }
}

impl From<TaskActions> for &str {
    fn from(action: TaskActions) -> Self {
        match action {
            TaskActions::StatusChange => "Change status",
            TaskActions::ChangeExecutor => "Change executor",
            TaskActions::ChangeRelease => "Change release",
        }
    }
}

impl TaskActions {
    /// Returns all available actions.
    pub fn get_actions() -> Vec<&'static str> {
        vec![
            Self::StatusChange.into(),
            Self::ChangeExecutor.into(),
            Self::ChangeRelease.into(),
        ]
    }

    /// Returns new action view based on `TaskActions` enum.
    pub fn get_view(self, cursive: &mut Cursive) -> impl ActionView {
        match self {
            TaskActions::StatusChange => ChangeStatusActionView::new(cursive),
            TaskActions::ChangeExecutor => ChangeStatusActionView::new(cursive),
            TaskActions::ChangeRelease => ChangeStatusActionView::new(cursive),
        }
    }
}
