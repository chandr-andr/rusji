use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{Dialog, NamedView, ResizedView, SelectView, ViewRef},
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::{
    jira::{
        common::views::{
            ButtonView, ChangeJiraView, JiraViewWithName, JiraWithDialogView,
            ToggleableView,
        },
        constance::INNER_CENTER_TOP_VIEW_ALIGN,
        tasks::views::InfoView,
        utils::helpers::calculate_view_size,
    },
    jira_data::JiraData,
};

#[derive(ViewWrapper)]
pub struct ChangeTransitionActionView {
    inner_view: NamedView<ResizedView<Dialog>>,
}

impl ToggleableView for ChangeTransitionActionView {}

impl ChangeJiraView for ChangeTransitionActionView {}

impl ButtonView for ChangeTransitionActionView {
    fn inner_view(self) -> NamedView<ResizedView<Dialog>> {
        self.inner_view
    }
}

impl JiraViewWithName for ChangeTransitionActionView {
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
}

impl JiraWithDialogView for ChangeTransitionActionView {
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
    /// Creates new ChangeStatusActionView view.
    ///
    /// Gets [`crate::jira_data::JiraData`] as a clone,
    /// Then gets available task statuses from selected task.
    ///
    /// After adds this task statuses to the new select view.
    pub fn new(cursive: &mut Cursive) -> Self
    where
        Self: Sized,
    {
        Self::toggle_on_view(cursive);
        let jira_data: Arc<RwLock<JiraData>> = cursive
            .user_data()
            .map(|jira_data: &mut Arc<RwLock<JiraData>>| Arc::clone(jira_data))
            .unwrap();

        let jira_data_guard = jira_data.read().unwrap();

        let jira_task = jira_data_guard.get_selected_task();

        let mut select_view = SelectView::<String>::new() // TODO: Rewrite as a separate view.
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, transaction_name: &str| {
                {
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
                }

                {
                    let jira_data: &mut Arc<RwLock<JiraData>> =
                        cursive.user_data().unwrap();
                    let mut jira_data_guard = jira_data.write().unwrap();
                    jira_data_guard.update_selected_issue();
                }

                InfoView::get_view(cursive).update_view_content(cursive);
            });

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
                .fixed_size(calculate_view_size(cursive, 3, 7))
                .with_name(Self::main_dialog_name()),
        }
    }

    fn change_status(cursive: &mut Cursive, transition_name: &str) {
        Self::toggle_off_view(cursive);
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
