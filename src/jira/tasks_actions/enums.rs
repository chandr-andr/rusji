use std::str::FromStr;

use cursive::{
    views::{Dialog, NamedView, ResizedView},
    Cursive,
};

use crate::jira::common::{
    buttons_variants::ButtonVariant, views::ButtonView,
};

use super::{
    change_assignee_views::ChangeAssigneeView,
    change_story_points_view::ChangeSPView,
    change_transition_view::ChangeTransitionActionView,
};

#[derive(Clone, Copy)] // TODO: remove Clone, Copy
pub enum TaskActions {
    StatusChange,
    ChangeAssignee,
    ChangeStoryPoints,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TaskActionParseError;

impl FromStr for TaskActions {
    type Err = TaskActionParseError;
    fn from_str(str_action: &str) -> Result<Self, Self::Err> {
        match str_action {
            "Change status" => Ok(TaskActions::StatusChange),
            "Change assignee" => Ok(TaskActions::ChangeAssignee),
            "Change story points" => Ok(TaskActions::ChangeStoryPoints),
            _ => Err(TaskActionParseError {}),
        }
    }
}

impl<'a> From<TaskActions> for &'a str {
    fn from(action: TaskActions) -> Self {
        match action {
            TaskActions::StatusChange => "Change status",
            TaskActions::ChangeAssignee => "Change assignee",
            TaskActions::ChangeStoryPoints => "Change story points",
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
            Self::ChangeStoryPoints.into(),
        ]
    }

    /// Returns new action view based on `TaskActions` enum.
    pub fn get_view(
        self,
        cursive: &mut Cursive,
    ) -> NamedView<ResizedView<Dialog>> {
        match self {
            TaskActions::StatusChange => {
                ChangeTransitionActionView::new(cursive).inner_view()
            }
            TaskActions::ChangeAssignee => {
                ChangeAssigneeView::new(cursive).inner_view()
            }
            TaskActions::ChangeStoryPoints => {
                ChangeSPView::new(cursive).inner_view()
            }
        }
    }
}
