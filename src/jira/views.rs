use cursive::{
    views::{
        Dialog,
        SelectView,
        ScrollView,
        LinearLayout,
        ViewRef,
        EditView,
        DummyView,

    },
    view::{
        Nameable,
        ViewWrapper,
        Resizable,
    },
    Cursive
};

use super::constance::{INNER_LEFT_TOP_VIEW_ALIGN, TASKS_SELECT_VIEW_NAME, INNER_CENTER_TOP_VIEW_ALIGN, PROJECTS_SELECT_VIEW_NAME, PROJECTS_SEARCH_VIEW_NAME, ACTIONS_SELECT_VIEW_NAME};
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
            );

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
        let projects = cursive_data.update_return_projects();
        select_project_view.clear();
        select_project_view.add_all_str(projects);
        cursive.set_user_data(cursive_data)
    }

    fn on_enter_search_project(cursive: &mut Cursive, project_subname: &str) {
        let cursive_data: &CursiveJiraData = cursive.user_data().unwrap();
        let fit_projects = cursive_data
            .jira_data
            .find_project_by_subname(project_subname);

        let fit_projects_select_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .on_submit(Self::pop_layout_show_tasks);

        let fit_projects_dialog = Dialog::new()
            .title("Select project")
            .content(fit_projects_select_view.with_all_str(fit_projects))
            .with_name(PROJECTS_SEARCH_VIEW_NAME);

        cursive.add_layer(fit_projects_dialog);
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

    fn pop_layout_show_tasks(cursive: &mut Cursive, project_name: &str) {
        cursive.pop_layer();
        Self::show_tasks(cursive, project_name);
    }
}

pub(crate) struct TasksView {
    inner_view: Dialog,
}

impl Default for TasksView {
    fn default() -> Self {
        let inner_tasks_view = SelectView::<String>::new()
            .align(INNER_LEFT_TOP_VIEW_ALIGN)
            .on_submit(Self::show_info_on_select);

        Self {
            inner_view: Dialog::new()
                .title("Choose issue")
                .padding_lrtb(1, 1, 1, 1)
                .content(
                    ScrollView::new(
                        inner_tasks_view.with_name(TASKS_SELECT_VIEW_NAME),
                    )
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