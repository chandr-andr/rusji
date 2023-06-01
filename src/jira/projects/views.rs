use std::sync::{Arc, RwLock};

use cursive::views::TextView;
use cursive::View;
use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{
        Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView,
        SelectView, ViewRef,
    },
    Cursive,
};
use rusji_derive::ViewWrapper;

use crate::errors::RusjiError;
use crate::jira::bottom_menu::data::Button;
use crate::jira::tasks::data::JiraIssues;
use crate::jira::{
    common::views::JiraView, constance::INNER_CENTER_TOP_VIEW_ALIGN,
    tasks::views::TasksView,
};
use crate::jira_data::JiraData;

/// Struct for view with Jira projects.
///
/// Has inner view.
#[derive(ViewWrapper)]
pub(crate) struct ProjectsView {
    inner_view: NamedView<Dialog>,
}

impl Default for ProjectsView {
    /// Creates SelectView in ScrollView,
    ///
    /// EditView in Dialog for search field
    ///
    /// and main dialog view for all views told before
    /// with LinearLayout in content to aggregate all of this.
    fn default() -> Self {
        let projects_select_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, selected_project: &str| {
                let jira_data: Arc<RwLock<JiraData>> = cursive
                    .user_data()
                    .map(|jira_data: &mut Arc<RwLock<JiraData>>| {
                        jira_data.clone()
                    })
                    .unwrap();

                {
                    let mut jira_guard = jira_data.write().unwrap();
                    jira_guard.set_selected_project(selected_project);

                    let client_clone = jira_guard.client.clone();
                    let project_key = jira_guard.get_selected_project_key();

                    let jira_tasks = {
                        let project_key_clone = project_key.clone();
                        jira_guard.thread_pool.evaluate(
                            move || -> Result<JiraIssues, RusjiError> {
                                JiraIssues::new(
                                    client_clone,
                                    project_key_clone.as_str(),
                                )
                            },
                        )
                    };
                    let jira_tasks_result = jira_tasks.await_complete();

                    jira_guard.update_tasks(jira_tasks_result);
                }

                TasksView::get_view(cursive).update_view_content(cursive);
            })
            .with_name(Self::select_view_name());
        let projects_scroll_view = ScrollView::new(projects_select_view);

        let search_project_dialog = Dialog::new()
            .title("Search project by name")
            .content(EditView::new().on_edit(|cursive, text, _cursor| {
                ProjectsView::on_enter_search_project(cursive, text)
            }))
            .with_name(Self::search_view_name());

        let dialog = Dialog::new()
            .title("Choose project")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(search_project_dialog)
                    .child(DummyView)
                    .child(projects_scroll_view),
            )
            .with_name(Self::main_dialog_name());

        Self { inner_view: dialog }
    }
}

impl JiraView for ProjectsView {
    /// Returns string with name for `ProjectsView`.
    fn view_name() -> String {
        String::from("ProjectsView")
    }

    /// Returns instance of `ProjectsView` from `Cursive` app.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    /// Returns name of the view with main ProjectsView layout - dialog.
    fn main_dialog_name() -> String {
        String::from("ProjectDialogView")
    }

    /// Returns the view with field for project search.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }

    /// Updates the projects names in SelectView.
    ///
    /// Tries to get new vector of projects from JiraData.
    /// If success clear SelectView and add new data, else add
    /// BadConnectionView with an error message.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        let mut select_project_view: ViewRef<SelectView> =
            self.get_select_view();

        let jira_data: Arc<RwLock<JiraData>> =
            cursive.take_user_data().unwrap();
        let jira_clone = jira_data.clone();

        let jira_data_guard = jira_clone.write().unwrap();

        let projects_names = jira_data_guard.get_projects_names();
        if projects_names.is_empty() {
            cursive.add_layer(
                Dialog::new()
                    .title("No projects")
                    .content(TextView::new("Can't find projects"))
                    .button("Ok", |cursive| {
                        cursive.pop_layer();
                    }),
            )
        } else {
            select_project_view.clear();
            select_project_view.add_all_str(projects_names);
        }
        cursive.set_user_data(jira_data);
    }

    /// Extends view content with passed `content`.
    fn add_content_to_view(&mut self, content: Vec<&str>) {
        let mut select_view = self.get_select_view();
        select_view.add_all_str(content);
    }
}

impl ProjectsView {
    /// Returns name of the view with list of projects names.
    pub fn select_view_name() -> String {
        String::from("ProjectSelectView")
    }

    /// Returns name of the view with search field.
    pub fn search_view_name() -> String {
        String::from("ProjectsSearchView")
    }

    /// Returns the view with list of projects names.
    fn get_select_view(&mut self) -> ViewRef<SelectView> {
        self.get_main_dialog()
            .find_name(&Self::select_view_name())
            .unwrap()
    }

    /// Gets input string from EditView as `project_subname`
    /// and tries to find suitable projects.
    ///
    /// If search result is empty just clear view with projects
    /// else show names of suitable projects.
    fn on_enter_search_project(cursive: &mut Cursive, project_subname: &str) {
        let mut select_project_view: ViewRef<SelectView> =
            ProjectsView::get_view(cursive).get_select_view();

        let jira_data: Arc<RwLock<JiraData>> =
            cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();

        let guard_jira_data = jira_data_clone.read().unwrap();
        let fit_projects =
            guard_jira_data.find_project_by_subname(project_subname);

        if fit_projects.is_empty() {
            select_project_view.clear();
        } else {
            select_project_view.clear();
            select_project_view.add_all_str(fit_projects);
        };
        cursive.set_user_data(jira_data);
    }
}
