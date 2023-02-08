use std::sync::{Arc, RwLock};

use cursive::View;
use cursive::{
    view::{Finder, Nameable, Scrollable, ViewWrapper},
    views::{
        Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView, SelectView, TextView,
        ViewRef,
    },
    Cursive,
};

use crate::jira::common::views::JiraView;
use crate::jira::constance::INNER_LEFT_TOP_VIEW_ALIGN;
use crate::jira::tasks_actions::views::MainActionsView;
use crate::jira_data::JiraData;

use super::data::JiraTask;

pub(crate) struct TasksView {
    inner_view: NamedView<Dialog>,
}

impl Default for TasksView {
    /// Creates Dialog with LinearLayout inside
    /// LinearLayout consists of the view for display tasks
    /// and the edit view to allow search throught tasks.
    fn default() -> Self {
        let search_task_view = {
            let layout = LinearLayout::vertical()
                .child(TextView::new("Press <Enter> if not found."))
                .child(
                    EditView::new()
                        .on_edit(|cursive: &mut Cursive, task_name: &str, _: usize| {
                            Self::get_view(cursive).on_enter_task_search(cursive, task_name)
                        })
                        .on_submit(|cursive: &mut Cursive, task_key: &str| {
                            let jira_data: &mut Arc<RwLock<JiraData>> =
                                cursive.user_data().unwrap();

                            {
                                let mut jira_data_guard = jira_data.write().unwrap();

                                jira_data_guard.set_selected_task(task_key);
                                let task_key = &jira_data_guard.selected_task;

                                let task = JiraTask::new(jira_data_guard.client.clone(), task_key);

                                jira_data_guard.add_new_task(task);
                            }

                            InfoView::get_view(cursive).update_view_content(cursive);
                            MainActionsView::get_view(cursive).update_view_content(cursive);
                            Self::get_view(cursive).on_enter_task_search(cursive, task_key);
                        })
                        .with_name(Self::search_view_name()),
                );

            Dialog::new().title("Search task by name").content(layout)
        };

        let inner_tasks_view = SelectView::<String>::new()
            .align(INNER_LEFT_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, task_name: &str| {
                let jira_data: &mut Arc<RwLock<JiraData>> = cursive.user_data().unwrap();

                jira_data.write().unwrap().set_selected_task(task_name);

                InfoView::get_view(cursive).show_info_on_select(cursive, task_name);
                MainActionsView::get_view(cursive).update_view_content(cursive);
            })
            .with_name(Self::select_view_name())
            .scrollable();

        Self {
            inner_view: Dialog::new()
                .title("Choose issue")
                .padding_lrtb(1, 1, 1, 1)
                .content(
                    LinearLayout::vertical()
                        .child(search_task_view)
                        .child(inner_tasks_view),
                )
                .with_name(Self::main_dialog_name()),
        }
    }
}

impl ViewWrapper for TasksView {
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

impl JiraView for TasksView {
    /// Returns name of the TasksView.
    fn view_name() -> String {
        String::from("TasksView")
    }

    /// Returns instance of the TasksView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    /// Returns name of the main Dialog in TasksView.
    fn main_dialog_name() -> String {
        String::from("TasksDialogName")
    }

    /// Returns instance of the main Dialog in TasksView.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }

    /// Updates SelectView in TasksView with data from JiraData.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        let mut tasks_select_view: ViewRef<SelectView> = self.get_select_view();

        let jira_data: Arc<RwLock<JiraData>> = cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();

        let jira_guard = jira_data_clone.read().unwrap();

        match jira_guard.get_selected_project().tasks_names() {
            Some(tasks_names) => {
                tasks_select_view.clear();
                tasks_select_view.add_all_str(tasks_names);
                cursive.focus_name(&TasksView::view_name()).unwrap();
            }
            None => cursive.add_layer(
                Dialog::new()
                    .title("No tasks")
                    .content(TextView::new("No tasks in this project"))
                    .button("Ok", |cursive| {
                        cursive.pop_layer();
                    }),
            ),
        }
        cursive.set_user_data(jira_data);
    }

    /// Adds new content to SelectView from passed `content`.
    fn add_content_to_view(&mut self, content: Vec<&str>) {
        self.get_select_view().add_all_str(content);
    }
}

impl TasksView {
    /// Returns name of the SelectView in TasksView.
    pub fn select_view_name() -> String {
        String::from("TasksSelectView")
    }

    /// Returns name of the EditView in TasksView.
    pub fn search_view_name() -> String {
        String::from("TasksSearchName")
    }

    /// Returns instance of the SelectView in TasksView.
    pub fn get_select_view(&mut self) -> ViewRef<SelectView> {
        self.get_main_dialog()
            .find_name(Self::select_view_name().as_str())
            .unwrap()
    }

    /// Tries to find task to display it.
    fn on_enter_task_search(&mut self, cursive: &mut Cursive, task_subname: &str) {
        let jira_data: Arc<RwLock<JiraData>> = cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();
        let mut tasks_select_view: ViewRef<SelectView> = self.get_select_view();

        let jira_data_guard = jira_data_clone.read().unwrap();
        let fit_tasks =
            jira_data_guard.find_task_by_subname(task_subname, &jira_data_guard.selected_project);

        if fit_tasks.is_empty() {
            tasks_select_view.clear();
        } else {
            tasks_select_view.clear();
            for task in fit_tasks {
                tasks_select_view.add_item_str(format!("{} -- {}", task.key, task.summary));
            }
        }
        cursive.set_user_data(jira_data);
    }
}

pub(crate) struct InfoView {
    inner_view: NamedView<Dialog>,
}

impl Default for InfoView {
    fn default() -> Self {
        Self::new("Choose task", "")
    }
}

impl ViewWrapper for InfoView {
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

impl JiraView for InfoView {
    /// Returns name of the InfoView.
    fn view_name() -> String {
        String::from("InfoView")
    }

    /// Returns the instance of the InfoView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }

    /// Returns name of the main dialog.
    fn main_dialog_name() -> String {
        String::from("InfoViewDialog")
    }

    /// Returns the instance of the main Dialog.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(Self::main_dialog_name().as_str()).unwrap()
    }

    /// Updates view with task information.
    ///
    /// In fact, just recreate InfoView without data and
    /// add it to InfoLayout.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        let jira_data: &mut Arc<RwLock<JiraData>> = cursive.user_data().unwrap();
        let jira_data_guard = jira_data.write().unwrap();
        let task = {
            let selected_task = &jira_data_guard.selected_task;
            jira_data_guard
                .get_selected_project()
                .get_task(selected_task)
        };
        self.get_main_dialog()
            .set_content(Self::make_inner_view(&task.summary, &task.description));
    }

    /// Does the same as `set_view_content` method.
    fn add_content_to_view(&mut self, _content: Vec<&str>) {}
}

impl InfoView {
    fn new(summary: &str, description: &str) -> Self {
        let dialog = Dialog::new()
            .title("Task information")
            .content(Self::make_inner_view(summary, description))
            .with_name(Self::main_dialog_name());

        Self { inner_view: dialog }
    }

    fn make_inner_view(summary: &str, description: &str) -> LinearLayout {
        LinearLayout::vertical()
            .child(InfoView::make_summary_dialog(summary))
            .child(DummyView)
            .child(InfoView::make_description_dialog(description))
    }

    fn make_summary_dialog(summary: &str) -> Dialog {
        Dialog::new()
            .title("Task")
            .content(cursive_markup::MarkupView::html(summary).with_name("summary_task_view"))
    }

    fn make_description_dialog(description: &str) -> Dialog {
        Dialog::new()
            .title("Description")
            .padding_lrtb(1, 1, 1, 1)
            .content(ScrollView::new(
                cursive_markup::MarkupView::html(description).with_name("description_task_view"),
            ))
    }

    /// Shows task information in InfoView.
    fn show_info_on_select(&mut self, cursive: &mut Cursive, task_name: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> = cursive.user_data().unwrap();
        let task_key: Vec<&str> = task_name.split(" -- ").collect();

        let jira_data_guard = jira_data.read().unwrap();
        let task = jira_data_guard
            .get_project(jira_data_guard.selected_project.clone().as_str())
            .get_task(task_key[0]);

        self.get_main_dialog()
            .set_content(Self::make_inner_view(&task.summary, &task.description));
    }
}
