use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{Dialog, NamedView, ScrollView, SelectView, ViewRef},
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::{
    jira::{
        common::views::{ActionView, JiraView},
        constance::INNER_CENTER_TOP_VIEW_ALIGN,
    },
    jira_data::JiraData,
};

use super::enums::TaskActions;

use std::str::FromStr;

#[derive(ViewWrapper)]
pub struct MainActionsView {
    inner_view: NamedView<Dialog>,
}

impl Default for MainActionsView {
    fn default() -> Self {
        let inner_action_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, action_name: &str| {
                let action: TaskActions =
                    TaskActions::from_str(action_name).unwrap();
                Self::get_view(cursive)
                    .add_certain_action_view(cursive, action);
            })
            .with_name(Self::select_view_name());

        Self {
            inner_view: Dialog::new()
                .title("Available action")
                .content(ScrollView::new(inner_action_view).full_height())
                .with_name(Self::main_dialog_name()),
        }
    }
}

impl JiraView for MainActionsView {
    /// Returns name of the MainActionsView.
    fn view_name() -> String {
        "MainActionsView".into()
    }

    /// Returns instance of the MainActionsView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    /// Returns name of the main Dialog in MainActionsView.
    fn main_dialog_name() -> String {
        "ActionsDialogName".into()
    }

    /// Returns instance of the main Dialog in MainActionsView.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }

    /// Updates SelectView in MainActionsView with data from JiraData.
    fn update_view_content(&mut self, _: &mut Cursive) {
        let mut select_view: ViewRef<SelectView> = self.get_select_view();
        select_view.clear();
        select_view.add_all_str(TaskActions::get_actions());
    }

    /// Adds new content to SelectView from passed `content`.
    fn add_content_to_view(&mut self, _: Vec<&str>) {}
}

impl MainActionsView {
    /// Returns name of the SelectView in MainActionsView.
    pub fn select_view_name() -> String {
        String::from("ActionsSelectView")
    }

    /// Returns instance of the SelectView in MainActionsView.
    pub fn get_select_view(&mut self) -> ViewRef<SelectView> {
        self.get_main_dialog()
            .find_name(Self::select_view_name().as_str())
            .unwrap()
    }

    /// Adds new view to main screen.
    ///
    /// Based on selected action.
    fn add_certain_action_view(
        &self,
        cursive: &mut Cursive,
        action: TaskActions,
    ) {
        let action_view = action.get_view(cursive);
        cursive.add_layer(action_view);
    }
}

#[derive(ViewWrapper)]
pub struct ChangeTransitionActionView {
    inner_view: NamedView<Dialog>,
}

impl ActionView for ChangeTransitionActionView {
    /// Creates new ChangeStatusActionView view.
    ///
    /// Gets [`crate::jira_data::JiraData`] as a clone,
    /// Then gets available task statuses from selected task.
    ///
    /// After adds this task statuses to the new select view.
    fn new(cursive: &mut Cursive) -> Self {
        let jira_data: Arc<RwLock<JiraData>> = cursive
            .user_data()
            .map(|jira_data: &mut Arc<RwLock<JiraData>>| Arc::clone(jira_data))
            .unwrap();
        let jira_data_guard = jira_data.read().unwrap();
        let jira_task = jira_data_guard.get_selected_task();

        let mut select_view = SelectView::<String>::new().on_submit(
            |cursive: &mut Cursive, transaction_name: &str| {
                Self::change_status(cursive, transaction_name);
                cursive.pop_layer();

                let jira_data: &mut Arc<RwLock<JiraData>> =
                    cursive.user_data().unwrap();
                let mut jira_data_guard = jira_data.write().unwrap();
                {
                    let client = jira_data_guard.client.clone();

                    jira_data_guard
                        .get_mut_selected_task()
                        .add_transitions(client)
                }
            },
        );
        select_view.add_all_str(
            jira_task
                .transitions
                .as_ref()
                .unwrap()
                .all_transitions_name(),
        );

        Self {
            inner_view: Dialog::new()
                .title("Choose new status")
                .content(select_view)
                .button("Back", |cursive: &mut Cursive| {
                    cursive.pop_layer();
                })
                .with_name(Self::main_dialog_name()),
        }
    }
}

impl JiraView for ChangeTransitionActionView {
    /// Returns name of the `ChangeStatusActionView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeStatusView".into()
    }

    /// Returns instance of `ChangeStatusActionView`
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    /// Returns name of the main Dialog in `ChangeStatusActionView`.
    fn main_dialog_name() -> String {
        "ChangeStatusDialogName".into()
    }

    /// Returns main dialog from the view.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ChangeTransitionActionView {
    fn change_status(cursive: &mut Cursive, transition_name: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();
        let jira_data_guard = jira_data.read().unwrap();

        let jira_task = jira_data_guard.get_selected_task();
        let transition_id = jira_task
            .transitions
            .as_ref()
            .unwrap()
            .get_transitions_id_by_name(transition_name);

        let client = jira_data_guard.client.read().unwrap();
        client
            .update_task_transition(&jira_task.key, transition_id)
            .unwrap();
    }
}
