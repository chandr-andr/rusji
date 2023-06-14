use std::sync::{Arc, RwLock};

use cursive::{view::Nameable, Cursive};

use crate::{
    jira::{
        common::views::JiraViewWithName,
        tasks_actions::main_view::ActionsView,
        utils::views::FailedAttemptView,
    },
    jira_data::JiraData,
};

pub fn build_tasks_action_view(cursive: &mut Cursive) {
    let is_task_exists = {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();
        let jira_data_guard = jira_data.read().unwrap();

        !jira_data_guard.selected_task.is_empty()
    };

    if is_task_exists {
        let task_actions_view =
            ActionsView::new(cursive).with_name(ActionsView::view_name());
        cursive.add_layer(task_actions_view);
    } else {
        let no_task_view = FailedAttemptView::new("Please select task");
        cursive.add_layer(no_task_view);
    }
}
