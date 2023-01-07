use cursive::{
    views::{
        Dialog,
        SelectView,
        ScrollView,
        LinearLayout,
        ViewRef,
        EditView,
        DummyView,
        TextView, NamedView,
    },
    view::{
        Nameable,
        ViewWrapper,
        Resizable,
        Scrollable, Finder,
    },
    Cursive,
};
use cursive::View;

use super::constance::{
    INNER_LEFT_TOP_VIEW_ALIGN,
    TASKS_SELECT_VIEW_NAME,
    INNER_CENTER_TOP_VIEW_ALIGN,
    ACTIONS_SELECT_VIEW_NAME,
    TASKS_SEARCH_VIEW_NAME,
    INFO_LAYOUT_VIEW_NAME,
};
use super::jira_data::CursiveJiraData;

/// Trait for all Jira views.
pub trait JiraView {
    /// Returns name of the view.
    fn view_name() -> String;
    /// Returns instance of class from cursive app.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self>;
    /// Updates view content with [`super::jira_data::JiraData`] methods.
    fn update_view_content(&mut self, cursive: &mut Cursive);
    /// Updates view content with passed `content`.
    fn set_view_content(&self, cursive: &mut Cursive, content: Vec<&str>);
    /// Extends view content with passed `content`.
    fn add_content_to_view(&self, cursive: &mut Cursive, content: Vec<&str>);
}

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
            F: FnOnce(&Self::V) -> R {
                Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where
            F: FnOnce(&mut Self::V) -> R {
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
            .on_submit(Self::show_tasks)
            .with_name(Self::select_view_name());
        let projects_scroll_view = ScrollView::new(
            projects_select_view,
        );

        let search_project_dialog = Dialog::new()
            .title("Search project by name")
            .content(
                EditView::new()
                    .on_edit(|cursive, text, _cursor| {
                        ProjectsView::on_enter_search_project(cursive, text)
                    })
            )
            .with_name(Self::search_view_name());

        let dialog = Dialog::new()
            .title("Choose project")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(search_project_dialog)
                    .child(DummyView)
                    .child(projects_scroll_view)
            ).with_name(Self::main_dialog_name());

        Self {
            inner_view: dialog,
        }
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

    /// Updates projects names in the view content.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        self.update_projects(cursive)
    }

    /// Sets new content to the view from passed `content`.
    fn set_view_content(&self, cursive: &mut Cursive, content: Vec<&str>) {
        let mut select_view = Self::get_view(cursive).get_select_view();
        select_view.clear();
        select_view.add_all_str(content);
    }

    /// Extends view content with passed `content`.
    fn add_content_to_view(&self, cursive: &mut Cursive, content: Vec<&str>) {
        let mut select_view = Self::get_view(cursive).get_select_view();
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

    /// Returns name of the view with main ProjectsView layout - dialog.
    fn main_dialog_name() -> String {
        String::from("ProjectDialogView")
    }

    /// Returns the view with list of projects names.
    fn get_select_view(&mut self) -> ViewRef<SelectView> {
        self.get_dialog_view().find_name(&Self::select_view_name()).unwrap()
    }

    /// Returns the view with list of projects names.
    fn get_search_view(&mut self) -> ViewRef<EditView> {
        self.get_dialog_view().find_name(&Self::search_view_name()).unwrap()
    }

    /// Returns the view with field for project search.
    fn get_dialog_view(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
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
            },
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
        let cursive_data: CursiveJiraData = cursive.take_user_data().unwrap();
        let mut select_project_view: ViewRef<SelectView> = ProjectsView::get_view(cursive)
            .get_select_view();
        let jira_data = &cursive_data.jira_data;
        let fit_projects = jira_data.find_project_by_subname(project_subname);

        if fit_projects.len() == 0 {
            select_project_view.clear();
        } else {
            select_project_view.clear();
            select_project_view.add_all_str(fit_projects);
        }
        cursive.set_user_data(cursive_data);
    }

    /// Adds tasks.
    fn show_tasks(cursive: &mut Cursive, project_name: &str) {
        let mut tasks_view: ViewRef<SelectView> = cursive
            .find_name(TASKS_SELECT_VIEW_NAME)
            .unwrap();
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        cursive_data.selected_project = project_name.to_string();

        let project_tasks = cursive_data.update_return_tasks(project_name);

        tasks_view.clear();
        tasks_view.add_all_str(project_tasks);
        tasks_view.sort();

        cursive.focus_name("tasks_view").unwrap();
    }
}

pub(crate) struct TasksView {
    inner_view: Dialog,
}

impl Default for TasksView {
    fn default() -> Self {
        let search_task_view = TasksView::make_task_find_dialog();

        let inner_tasks_view = SelectView::<String>::new()
            .align(INNER_LEFT_TOP_VIEW_ALIGN)
            .on_submit(Self::show_info_on_select)
            .with_name(TASKS_SELECT_VIEW_NAME)
            .scrollable();

        let tasks_layout = LinearLayout::vertical()
            .child(search_task_view)
            .child(inner_tasks_view);

        Self {
            inner_view: Dialog::new()
                .title("Choose issue")
                .padding_lrtb(1, 1, 1, 1)
                .content(
                    tasks_layout,
                ),
        }
    }
}

impl ViewWrapper for TasksView {
    type V = Dialog;

    fn with_view<F, R>(&self, f: F) -> Option<R>
        where
            F: FnOnce(&Self::V) -> R {
                Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where
            F: FnOnce(&mut Self::V) -> R {
                Some(f(&mut self.inner_view))
    }
}

impl TasksView {
    fn show_info_on_select(cursive: &mut Cursive, task_name: &str) {
        let mut info_layout: ViewRef<LinearLayout> = cursive.find_name("info_layout").unwrap();
        let c_jira_data: &CursiveJiraData = cursive.user_data().unwrap();
        let task_key: Vec<&str> = task_name.split(" -- ").collect();

        let (summary, description) = c_jira_data
            .jira_data
            .get_task_description(&c_jira_data.selected_project, task_key[0]);

        let new_info_view = InfoView::new(summary, description);
        info_layout.clear();
        info_layout.add_child(new_info_view);
    }

    fn make_task_find_dialog() -> Dialog {
        let layout = LinearLayout::vertical()
            .child(TextView::new("Press <Enter> if not found."))
            .child(
                EditView::new()
                    .on_edit(Self::on_enter_task_search)
                    .on_submit(Self::make_http_search)
                    .with_name(TASKS_SEARCH_VIEW_NAME)
            );

        Dialog::new()
            .title("Search task by name")
            .content(layout)
    }

    fn on_enter_task_search(cursive: &mut Cursive, task_subname: &str, _: usize) {
        let cursive_data: CursiveJiraData = cursive.take_user_data().unwrap();
        let mut tasks_select_view: ViewRef<SelectView> = cursive
            .find_name(TASKS_SELECT_VIEW_NAME)
            .unwrap();
        let jira_data = &cursive_data.jira_data;
        let fit_tasks = jira_data.find_task_by_subname(
            task_subname,
            &cursive_data.selected_project,
        );
        if fit_tasks.is_empty() {
            tasks_select_view.clear();
        } else {
            tasks_select_view.clear();
            tasks_select_view.add_all_str(fit_tasks);
        }
        cursive.set_user_data(cursive_data);
    }

    fn make_http_search(cursive: &mut Cursive, task_key: &str) {
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        match cursive_data.jira_data.get_new_task(
            task_key,
            &cursive_data.selected_project,
            &cursive_data.encoded_creds,
        ) {
            Ok((summary, desc)) => {
                let mut info_layout: ViewRef<LinearLayout> = cursive.find_name(INFO_LAYOUT_VIEW_NAME).unwrap();
                let new_info_view = InfoView::new(summary.as_str(), desc.as_str());
                info_layout.clear();
                info_layout.add_child(new_info_view);
            },
            Err(_) => {
                cursive.add_layer(
                    Dialog::new()
                        .title("Task not found")
                        .button("Ok", |cursive| {
                            cursive.pop_layer();
                        })
                )
            }
        }
    }
}

pub(crate) struct InfoView {
    inner_view: Dialog
}

impl Default for InfoView {
    fn default() -> Self {
        InfoView::new("Choose task", "")
    }
}

impl ViewWrapper for InfoView {
    type V = Dialog;

    fn with_view<F, R>(&self, f: F) -> Option<R>
        where
            F: FnOnce(&Self::V) -> R {
                Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where
            F: FnOnce(&mut Self::V) -> R {
                Some(f(&mut self.inner_view))
    }
}

impl InfoView {
    fn new(summary: &str, description: &str) -> Self {
        let dialog = Dialog::new()
            .title("Task information")
            .content(
                LinearLayout::vertical()
                    .child(InfoView::make_summary_dialog(summary))
                    .child(DummyView)
                    .child(InfoView::make_description_dialog(description))
            );

        Self { inner_view: dialog }
    }

    fn make_summary_dialog(summary: &str) -> Dialog {
        Dialog::new()
        .title("Задача")
        .content(
            cursive_markup::MarkupView::html(summary)
                .with_name("summary_task_view")
        )
    }

    fn make_description_dialog(description: &str) -> Dialog {
        Dialog::new()
            .title("Описание")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ScrollView::new(
                    cursive_markup::MarkupView::html(description)
                        .with_name("description_task_view")
                )
            )
    }
}


pub struct ActionsView {
    inner_view: Dialog
}

impl Default for ActionsView {
    fn default() -> Self {
        let inner_action_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .with_name(ACTIONS_SELECT_VIEW_NAME);

        Self {
            inner_view: Dialog::new()
                .title("Choose action")
                .content(
                    ScrollView::new(inner_action_view).full_height()
                )
        }
    }
}

impl ViewWrapper for ActionsView {
    type V = Dialog;

    fn with_view<F, R>(&self, f: F) -> Option<R>
        where
            F: FnOnce(&Self::V) -> R {
                Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where
            F: FnOnce(&mut Self::V) -> R {
                Some(f(&mut self.inner_view))
    }
}

pub(crate) struct BadConnectionView {
    inner_view: Dialog,
}

impl ViewWrapper for BadConnectionView {
    type V = Dialog;

    fn with_view<F, R>(&self, f: F) -> Option<R>
        where
            F: FnOnce(&Self::V) -> R {
                Some(f(&self.inner_view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where
            F: FnOnce(&mut Self::V) -> R {
                Some(f(&mut self.inner_view))
    }
}

impl BadConnectionView {
    pub fn new<T>(
        error_text: &str,
        try_again_fn: T,
    ) -> Self
    where T: 'static + Fn(&mut Cursive) {
        Self {
            inner_view:
                Dialog::new()
                    .title("Connection error!")
                    .content(TextView::new(error_text))
                    .button("Try again", try_again_fn)
        }
    }
}
