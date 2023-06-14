use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{
        Dialog, NamedView, OnEventView, ResizedView, ScrollView, SelectView,
        ViewRef,
    },
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::jira::{
    common::{
        button::CallbackText,
        views::{JiraViewWithName, JiraWithDialogView, ToggleableView},
    },
    constance::INNER_CENTER_TOP_VIEW_ALIGN,
    utils::helpers::calculate_view_size,
};

use super::{
    buttons::{build_buttons, TasksActionsButtons},
    enums::TaskActions,
};

use std::str::FromStr;

#[derive(ViewWrapper)]
pub struct ActionsView {
    inner_view: NamedView<ResizedView<Dialog>>,
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
                .title("Available actions")
                .content(ScrollView::new(on_event_view))
                .fixed_size(calculate_view_size(cursive, 5, 7))
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

impl JiraViewWithName for ActionsView {
    /// Returns name of the MainActionsView.
    fn view_name() -> String {
        "MainActionsView".into()
    }

    /// Returns instance of the MainActionsView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for ActionsView {
    /// Returns name of the main Dialog in MainActionsView.
    fn main_dialog_name() -> String {
        "ActionsDialogName".into()
    }

    /// Returns instance of the main Dialog in MainActionsView.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl<'a> ActionsView {
    /// Returns name of the SelectView in MainActionsView.
    pub fn select_view_name() -> String {
        String::from("ActionsSelectView")
    }

    /// Adds new view to the main screen.
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
