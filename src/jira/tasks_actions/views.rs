use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{Dialog, NamedView, OnEventView, ScrollView, SelectView, ViewRef},
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::{
    jira::{
        common::{
            button::CallbackText,
            views::{ButtonView, JiraView, ToggleableView},
        },
        constance::INNER_CENTER_TOP_VIEW_ALIGN,
    },
    jira_data::JiraData,
};

use super::{
    buttons::{build_buttons, TasksActionsButtons},
    enums::TaskActions,
};

use std::str::FromStr;

#[derive(ViewWrapper)]
pub struct ActionsView {
    inner_view: NamedView<Dialog>,
}

impl ToggleableView for ActionsView {}

impl ActionsView {
    pub fn new(cursive: &mut Cursive) -> Self {
        Self::toggle_on_view(cursive);
        let select_view_callback_buttons = build_buttons(cursive);
        let inner_select_view =
            Self::build_select_view(&select_view_callback_buttons);
        let on_event_view = Self::build_on_event_view(
            inner_select_view,
            select_view_callback_buttons,
        );

        Self {
            inner_view: Dialog::new()
                .title("Available action")
                .content(ScrollView::new(on_event_view))
                .with_name(Self::main_dialog_name()),
        }
    }

    fn build_select_view(
        callback_buttons: &TasksActionsButtons,
    ) -> NamedView<SelectView> {
        SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, action_name: &str| {
                Self::on_submit_select_view(cursive, action_name);
            })
            .with_all_str(
                callback_buttons
                    .buttons
                    .iter()
                    .map(|button| button.display_text()),
            )
            .with_name(Self::select_view_name())
    }

    fn on_submit_select_view(cursive: &mut Cursive, action_name: &str) {
        let action_text: &str =
            action_name.split(" - ").collect::<Vec<&str>>()[1];
        let action: TaskActions = TaskActions::from_str(action_text).unwrap();
        Self::get_view(cursive).add_certain_action_view(cursive, action);
    }

    fn build_on_event_view<'a>(
        select_view: NamedView<SelectView>,
        callback_buttons: TasksActionsButtons<'a>,
    ) -> OnEventView<NamedView<SelectView>> {
        let mut on_event_view = OnEventView::new(select_view);

        for button in callback_buttons.buttons.into_iter() {
            on_event_view =
                on_event_view.on_event(button.event, button.action_fn)
        }

        on_event_view
    }
}

impl JiraView for ActionsView {
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

impl<'a> ActionsView {
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

impl ButtonView for ChangeTransitionActionView {
    /// Creates new ChangeStatusActionView view.
    ///
    /// Gets [`crate::jira_data::JiraData`] as a clone,
    /// Then gets available task statuses from selected task.
    ///
    /// After adds this task statuses to the new select view.
    fn new(cursive: &mut Cursive) -> Self
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

impl ToggleableView for ChangeTransitionActionView {}
