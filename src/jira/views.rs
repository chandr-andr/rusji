use cursive::{
    views::{
        Dialog,
        SelectView,
        ScrollView,
        LinearLayout,
        ViewRef,
        EditView,
        DummyView,
        TextView,
    },
    view::{
        Nameable,
        ViewWrapper,
        Resizable,
        Scrollable,
    },
    Cursive
};

use super::constance::{
    INNER_LEFT_TOP_VIEW_ALIGN,
    TASKS_SELECT_VIEW_NAME,
    INNER_CENTER_TOP_VIEW_ALIGN,
    PROJECTS_SELECT_VIEW_NAME,
    PROJECTS_SEARCH_VIEW_NAME,
    ACTIONS_SELECT_VIEW_NAME,
    TASKS_SEARCH_VIEW_NAME, TASKS_HTTP_SEARCH_VIEW_NAME,
};
use super::jira_data::CursiveJiraData;

pub(crate) struct ProjectsView {
    inner_view: Dialog
}

impl Default for ProjectsView {
    fn default() -> Self {
        let search_project_dialog = Dialog::new()
            .title("Search project by name")
            .content(
                EditView::new()
                    .on_submit(Self::on_enter_search_project)
            )
            .with_name(PROJECTS_SEARCH_VIEW_NAME);

        let projects_scroll_view = ScrollView::new(
            SelectView::<String>::new()
                .align(INNER_CENTER_TOP_VIEW_ALIGN)
                .on_submit(Self::show_tasks)
                .with_name(PROJECTS_SELECT_VIEW_NAME)
        );

        let dialog = Dialog::new()
            .title("Choose project")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(search_project_dialog)
                    .child(DummyView)
                    .child(projects_scroll_view)
            );

        Self { inner_view: dialog }
    }
}

impl ViewWrapper for ProjectsView {
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

impl ProjectsView {
    pub fn update_projects(cursive: &mut Cursive) {
        let mut cursive_data: CursiveJiraData = cursive.take_user_data().unwrap();

        let mut select_project_view: ViewRef<SelectView> = cursive
            .find_name(PROJECTS_SELECT_VIEW_NAME)
            .unwrap();
        match cursive_data.update_return_projects() {
            Ok(projects) => {
                select_project_view.clear();
                select_project_view.add_all_str(projects);
            },
            Err(_) => {
                let bad_view = BadConnectionView::new(
                    "Can't get projects from Jira.",
                    ProjectsView::pop_layer_and_update_projects,
                );
                cursive.add_layer(bad_view);
            }
        }
        cursive.set_user_data(cursive_data);
    }

    fn pop_layer_and_update_projects(cursive: &mut Cursive) {
        cursive.pop_layer();
        ProjectsView::update_projects(cursive);
    }

    fn on_enter_search_project(cursive: &mut Cursive, project_subname: &str) {
        let cursive_data: CursiveJiraData = cursive.take_user_data().unwrap();
        let jira_data = &cursive_data.jira_data;
        let fit_projects = jira_data.find_project_by_subname(project_subname);

        if fit_projects.len() == 0 {
            cursive.add_layer(
                Dialog::new()
                    .title("No projects found")
                    .button("OK", |cursive| {
                        cursive.pop_layer();
                        cursive.focus_name(PROJECTS_SEARCH_VIEW_NAME).unwrap();
                    })
            );
        } else {
            let fit_projects_select_view = SelectView::<String>::new()
                .align(INNER_CENTER_TOP_VIEW_ALIGN)
                .on_submit(|cursive, project_name: &str| {
                    cursive.pop_layer();
                    Self::show_tasks(cursive, project_name);
                });

            let fit_projects_dialog = Dialog::new()
                .title("Select project")
                .content(fit_projects_select_view.with_all_str(fit_projects))
                .button("Back", |cursive| {
                    cursive.pop_layer();
                    cursive.focus_name(PROJECTS_SEARCH_VIEW_NAME).unwrap();
                });

            cursive.add_layer(fit_projects_dialog);
        }
        cursive.set_user_data(cursive_data);
    }

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
        Dialog::new()
            .title("Search task by name")
            .content(
                EditView::new()
                    .on_submit(Self::on_enter_task_search)
                    .with_name(TASKS_SEARCH_VIEW_NAME)
            )
    }

    fn on_enter_task_search(cursive: &mut Cursive, task_subname: &str) {
        let cursive_data: &mut CursiveJiraData = cursive.user_data().unwrap();
        let jira_data = &cursive_data.jira_data;
        let fit_tasks = jira_data.find_task_by_subname(
            task_subname,
            &cursive_data.selected_project,
        );
        if fit_tasks.is_empty() {
            TasksView::make_failed_task_search_dialog(cursive);
        } else {
            let fit_tasks_select_view = SelectView::<String>::new()
                .align(INNER_CENTER_TOP_VIEW_ALIGN)
                .on_submit(|cursive, task_name: &str| {
                    cursive.pop_layer();
                    TasksView::show_info_on_select(cursive, task_name);
                });

            let fit_tasks_dialog = Dialog::new()
                .title("Select task")
                .content(fit_tasks_select_view.with_all_str(fit_tasks))
                .button("Back", |cursive| {
                    cursive.pop_layer();
                    cursive.focus_name(TASKS_SEARCH_VIEW_NAME).unwrap();
                });

            cursive.add_layer(fit_tasks_dialog);
        }
    }

    fn make_failed_task_search_dialog(cursive: &mut Cursive) {
        let not_found_task_text = TextView::new(
"
Can't find this task fast.
Do you want to search by API?
",
        );

        let search_edit_view = EditView::new()
            .with_name(TASKS_HTTP_SEARCH_VIEW_NAME)
            .min_width(40);

        let layout = LinearLayout::vertical()
            .child(not_found_task_text)
            .child(DummyView)
            .child(TextView::new("Enter full task name or just number of task"))
            .child(search_edit_view);

        let failed_task_search_dialog = Dialog::new()
            .title("Search failed")
            .content(layout)
            .button("No", |cursive| {
                cursive.pop_layer();
                cursive.focus_name(TASKS_SEARCH_VIEW_NAME).unwrap();
            })
            .button("Search", |cursive| {
                TasksView::make_http_task_search(cursive);
            }
            );

        cursive.add_layer(failed_task_search_dialog);
    }

    fn make_http_task_search(cursive: &mut Cursive) {
        todo!();
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