use std::sync::{Arc, RwLock};

use cursive::{
    views::{NamedView, Dialog, SelectView, ScrollView, ViewRef},
    view::{Nameable, Resizable, ViewWrapper, Finder},
    Cursive,
};

use crate::{jira::{constance::INNER_CENTER_TOP_VIEW_ALIGN, common::views::JiraView}, jira_data::JiraData};

use super::enums::Actions;

pub struct MainActionsView {
    inner_view: NamedView<Dialog>,
}

impl Default for MainActionsView {
    fn default() -> Self {
        let inner_action_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, action_name: &str| {
                let jira_data: Arc<RwLock<JiraData>> = cursive
                    .user_data()
                    .map(|jira_data: &mut Arc<RwLock<JiraData>>| Arc::clone(jira_data))
                    .unwrap();
                let jira_data_guard = jira_data.read().unwrap();
                let jira_task = jira_data_guard.get_selected_task();

                if let Some(task_types) = &jira_data_guard.task_types {
                    let task_statuses = task_types.get_available_task_statuses(
                        &jira_task.issuetype.name,
                    );
                    let action: Actions = action_name.into();
                    let change_status = ChangeStatusActionView::new(task_statuses);
                    cursive.add_layer(change_status)
                }
            })
            .with_name(Self::select_view_name());

        Self {
            inner_view: Dialog::new()
                .title("Available action")
                .content(ScrollView::new(inner_action_view).full_height())
                .with_name(Self::main_dialog_name())
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
        select_view.add_all_str(Actions::get_actions());
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
}

pub struct ChangeStatusActionView {
    inner_view: NamedView<Dialog>,
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
    pub fn new(task_statuses: Vec<&str>) -> Self {
        let mut select_view = SelectView::new();

        select_view.add_all_str(task_statuses);
        Self {
            inner_view: Dialog::new()
                .title("Choose new status")
                .content(select_view)
                .button("Back", |cursive: &mut Cursive| {
                    cursive.pop_layer();
                })
                .with_name(Self::main_dialog_name())
        }
    }
}