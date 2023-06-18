use std::sync::{Arc, RwLock};

use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{
        Dialog, EditView, LinearLayout, NamedView, ResizedView, SelectView,
        ViewRef,
    },
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::{
    jira::{
        common::views::{
            ButtonView, ChangeJiraView, JiraViewWithName, JiraWithDialogView,
            ToggleableView,
        },
        utils::helpers::calculate_view_size,
    },
    jira_data::JiraData,
};

use super::data::JiraUsers;

#[derive(ViewWrapper)]
pub struct ChangeAssigneeView {
    inner_view: NamedView<ResizedView<Dialog>>,
}

impl ToggleableView for ChangeAssigneeView {}

impl JiraViewWithName for ChangeAssigneeView {
    /// Returns name of the `ChangeAssigneeSearchView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeAssigneeSearchView".into()
    }

    /// Returns instance of `ChangeAssigneeSearchView`
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for ChangeAssigneeView {
    /// Returns name of the main Dialog in `ChangeAssigneeSearchView`.
    fn main_dialog_name() -> String {
        "ChangeAssigneeSearchViewDialog".into()
    }

    /// Returns main dialog from the view.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ChangeJiraView for ChangeAssigneeView {}

impl ButtonView for ChangeAssigneeView {
    fn inner_view(self) -> NamedView<ResizedView<Dialog>> {
        self.inner_view
    }
}

impl ChangeAssigneeView {
    pub fn new(cursive: &mut Cursive) -> Self {
        Self::toggle_on_view(cursive);
        Self {
            inner_view: Dialog::new()
                .title("Assignee search, press <enter>")
                .content(ChangeAssigneeInnerLayout::new().inner_layout)
                .fixed_size(calculate_view_size(cursive, 5, 7))
                .with_name(Self::main_dialog_name()),
        }
    }
}

struct ChangeAssigneeInnerLayout {
    inner_layout: LinearLayout,
}

impl ChangeAssigneeInnerLayout {
    fn new() -> Self {
        let change_assignee_inner_layout = LinearLayout::vertical()
            .child(ChangeAssigneeEditView::new("Assignee search"))
            .child(
                ChangeAssigneeSelectView::new()
                    .with_name(ChangeAssigneeSelectView::view_name()),
            );

        Self {
            inner_layout: change_assignee_inner_layout,
        }
    }
}

#[derive(ViewWrapper)]
struct ChangeAssigneeEditView {
    inner_view: NamedView<Dialog>,
}

impl JiraViewWithName for ChangeAssigneeEditView {
    /// Returns name of the `ChangeAssigneeEditView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeAssigneeEditView".into()
    }

    /// Returns instance of `ChangeAssigneeEditView`
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for ChangeAssigneeEditView {
    /// Returns name of the main Dialog in `ChangeAssigneeSearchView`.
    fn main_dialog_name() -> String {
        "ChangeAssigneeEditViewDialog".into()
    }

    /// Returns main dialog from the view.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ChangeAssigneeEditView {
    fn new<ToString>(dialog_title: ToString) -> Self
    where
        ToString: Into<String>,
    {
        let change_assignee_edit_view =
            EditView::new().on_submit(Self::on_submit_callback);
        Self {
            inner_view: Dialog::new()
                .title(dialog_title)
                .content(change_assignee_edit_view)
                .with_name(Self::main_dialog_name()),
        }
    }

    fn on_submit_callback(cursive: &mut Cursive, username: &str) {
        let users = {
            let jira_data: &mut Arc<RwLock<JiraData>> =
                cursive.user_data().unwrap();
            let jira_data_guard = jira_data.read().unwrap();

            let request_client = jira_data_guard.client.clone();
            let response =
                request_client.read().unwrap().get_jira_users(username);

            match response {
                Ok(response) => {
                    let serialized =
                        serde_json::from_str::<JiraUsers>(response.get_body());
                    match serialized {
                        Ok(serialized) => serialized,
                        Err(err) => {
                            print!("{}", err);
                            return;
                        }
                    }
                }
                Err(_) => return,
            }
        };

        let mut change_assignee_select_view =
            ChangeAssigneeSelectView::get_view(cursive);

        change_assignee_select_view.update_with_data(
            users
                .into_iter()
                .map(|user| {
                    format!(
                        "{} | {} | {}",
                        user.display_name, user.name, user.key
                    )
                })
                .collect(),
        );
    }
}

#[derive(ViewWrapper)]
struct ChangeAssigneeSelectView {
    pub inner_view: SelectView,
}

impl JiraViewWithName for ChangeAssigneeSelectView {
    /// Returns name of the `ChangeAssigneeSelectView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeAssigneeSelectView".into()
    }

    /// Returns instance of `ChangeAssigneeSelectView`
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl ChangeJiraView for ChangeAssigneeSelectView {}

impl ChangeAssigneeSelectView {
    pub fn new() -> Self {
        let change_assignee_select_view = SelectView::new();
        Self {
            inner_view: change_assignee_select_view,
        }
    }

    /// Update inner view with new_data data.
    pub fn update_with_data(&mut self, new_data: Vec<String>) {
        self.inner_view.clear();
        self.inner_view.add_all_str(new_data);
    }
}
