use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{Dialog, NamedView, ScrollView, SelectView, ViewRef},
    Cursive,
};

use crate::{
    jira::{
        common::views::{ActionView, JiraView},
        constance::INNER_CENTER_TOP_VIEW_ALIGN,
    },
    jira_data::JiraData,
};

use super::enums::TaskActions;

use std::str::FromStr;

pub struct MainActionsView {
    inner_view: NamedView<Dialog>,
}

impl Default for MainActionsView {
    fn default() -> Self {
        let inner_action_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, action_name: &str| {
                let action: TaskActions = TaskActions::from_str(action_name).unwrap();
                Self::get_view(cursive).add_certain_action_view(cursive, action);
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

impl ViewWrapper for MainActionsView {
    type V = NamedView<Dialog>;

    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        Some(f(&mut self.inner_view))
    }
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

    /// Adds new layout to main screnn.
    ///
    /// Based on selected action.
    fn add_certain_action_view(&self, cursive: &mut Cursive, action: TaskActions) {
        let action_view = action.get_view(cursive);
        cursive.add_layer(action_view);
    }
}

pub struct ChangeStatusActionView {
    inner_view: NamedView<Dialog>,
}

impl ActionView for ChangeStatusActionView {
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

        let mut select_view = SelectView::new();

        if let Some(task_types) = &jira_data_guard.task_types {
            let task_statuses = task_types.get_available_task_statuses(&jira_task.issuetype.name);
            select_view.add_all_str(task_statuses);
        }

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

impl ViewWrapper for ChangeStatusActionView {
    type V = NamedView<Dialog>;

    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        Some(f(&mut self.inner_view))
    }
}

impl JiraView for ChangeStatusActionView {
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

impl ChangeStatusActionView {
    fn change_status(&self, cursive: &mut Cursive, new_status_name: &str) {

    }
}