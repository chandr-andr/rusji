use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{
        Dialog, EditView, LinearLayout, NamedView, SelectView,
        ViewRef,
    },
    Cursive, View,
};
use rusji_derive::ViewWrapper;

use crate::jira::common::views::{
    ChangeJiraView, JiraViewWithName, JiraWithDialogView, ToggleableView,
};

#[derive(ViewWrapper)]
pub struct ChangeAssigneeSearchView {
    inner_view: NamedView<Dialog>,
}

impl ToggleableView for ChangeAssigneeSearchView {}

impl JiraViewWithName for ChangeAssigneeSearchView {
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

impl JiraWithDialogView for ChangeAssigneeSearchView {
    /// Returns name of the main Dialog in `ChangeAssigneeSearchView`.
    fn main_dialog_name() -> String {
        "ChangeAssigneeSearchViewDialog".into()
    }

    /// Returns main dialog from the view.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ChangeJiraView for ChangeAssigneeSearchView {}

impl Default for ChangeAssigneeSearchView {
    fn default() -> Self {
        Self {
            inner_view: Dialog::new()
                .title("Assignee search, press <enter>")
                .content(Self::build_select_view())
                // .content(
                // LinearLayout<Dialog<EditView><SelectView>>
                //)
                .with_name(Self::main_dialog_name()),
        }
    }
}

struct ChangeAssigneeInnerLayout {
    inner_layout: LinearLayout,
}

impl ChangeAssigneeInnerLayout {
    fn new() -> Self {
        let change_assignee_inner_layout = LinearLayout::vertical();

        Self {
            inner_layout: change_assignee_inner_layout,
        }
    }
}

impl ChangeAssigneeSearchView {
    fn build_select_view() -> SelectView {
        
        SelectView::new()
    }
}

#[derive(ViewWrapper)]
struct ChangeAssigneeEditView {
    inner_view: NamedView<Dialog>,
}

impl JiraViewWithName for ChangeAssigneeEditView {
    /// Returns name of the `ChangeAssigneeSearchView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeAssigneeEditView".into()
    }

    /// Returns instance of `ChangeAssigneeSearchView`
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
    fn new<ToString, Callback>(
        dialog_title: ToString,
        on_submit_callback: Callback,
    ) -> Self
    where
        ToString: Into<String>,
        Callback: Fn(&mut Cursive, &str) + 'static,
    {
        let change_assignee_edit_view =
            EditView::new().on_submit(on_submit_callback);
        Self {
            inner_view: Dialog::new()
                .title(dialog_title)
                .content(change_assignee_edit_view)
                .with_name(Self::main_dialog_name()),
        }
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
}
