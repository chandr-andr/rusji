use cursive::View;
use cursive::{
    view::{Finder, Nameable, ViewWrapper},
    views::{
        Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView, SelectView, ViewRef,
    },
    Cursive,
};

use crate::jira::jira_data::CursiveJiraData;
use crate::jira::utils::views::BadConnectionView;
use crate::jira::{
    common::views::JiraView, constance::INNER_CENTER_TOP_VIEW_ALIGN, tasks::views::TasksView,
};

/// Struct for view with Jira projects.
///
/// Has inner view.
///
/// This view can be used as regular view because it implements ViewWrapper
pub(crate) struct ProjectsView {
    inner_view: NamedView<Dialog>,
}

impl ViewWrapper for ProjectsView {
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

    fn wrap_call_on_any<'a>(
        &mut self,
        selector: &cursive::view::Selector<'_>,
        callback: cursive::event::AnyCb<'a>,
    ) {
        self.with_view_mut(|v| v.call_on_any(selector, callback));
    }
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
                Self::set_selected_project(cursive, selected_project);
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

    /// Updates projects names in the view content.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        self.update_projects(cursive)
    }

    /// Sets new content to the view from passed `content`.
    fn set_view_content(&mut self, content: Vec<&str>) {
        let mut select_view = self.get_select_view();
        select_view.clear();
        select_view.add_all_str(content);
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

    /// Returns the view with list of projects names.
    fn get_search_view(&mut self) -> ViewRef<EditView> {
        self.get_main_dialog()
            .find_name(&Self::search_view_name())
            .unwrap()
    }

    fn set_selected_project(cursive: &mut Cursive, selected_project: &str) {
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        cursive_data.selected_project = selected_project.to_string();
    }

    /// Updates the projects names in SelectView.
    ///
    /// Tries to get new vector of projects from JiraData.
    /// If success clear SelectView and add new data, else add
    /// BadConnectionView with an error message.
    fn update_projects(&mut self, cursive: &mut Cursive) {
        let mut select_project_view: ViewRef<SelectView> = self.get_select_view();
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        match cursive_data.update_return_projects() {
            Ok(projects) => {
                select_project_view.clear();
                select_project_view.add_all_str(projects);
            }
            Err(_) => {
                let bad_view = BadConnectionView::new(
                    "Can't get projects from Jira.",
                    |cursive: &mut Cursive| {
                        cursive.pop_layer();
                        Self::get_view(cursive).update_view_content(cursive)
                    },
                );
                cursive.add_layer(bad_view);
            }
        }
    }

    /// Gets input string from EditView as `project_subname`
    /// and tries to find suitable projects.
    ///
    /// If search result is empty just clear view with projects
    /// else show names of suitable projects.
    fn on_enter_search_project(cursive: &mut Cursive, project_subname: &str) {
        let mut select_project_view: ViewRef<SelectView> =
            ProjectsView::get_view(cursive).get_select_view();
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        let jira_data = &cursive_data.jira_data;
        let fit_projects = jira_data.find_project_by_subname(project_subname);

        if fit_projects.is_empty() {
            select_project_view.clear();
        } else {
            select_project_view.clear();
            select_project_view.add_all_str(fit_projects);
        }
    }
}
