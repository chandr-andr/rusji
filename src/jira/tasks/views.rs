use std::sync::{Arc, RwLock};

use cursive::view::Resizable;

use cursive::View;
use cursive::{
    view::{Finder, Nameable, Scrollable, ViewWrapper},
    views::{
        Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView,
        SelectView, TextView, ViewRef,
    },
    Cursive,
};

use rusji_derive::ViewWrapper;

use crate::jira::common::views::{
    ChangeJiraView, JiraViewWithName, JiraWithDialogView,
};
use crate::jira::constance::INNER_LEFT_TOP_VIEW_ALIGN;
use crate::jira::utils::views::FailedAttemptView;
use crate::jira_data::JiraData;

use super::data::{IssueBaseInfo, IssueBaseInfoField, JiraIssue};

#[derive(ViewWrapper)]
pub(crate) struct TasksView {
    inner_view: NamedView<Dialog>,
}

impl TasksView {
    /// Returns name of the EditView in TasksView.
    pub fn search_view_name() -> String {
        "TasksSearchName".into()
    }

    /// Returns instance of the SelectView in TasksView.
    pub fn get_select_view(&mut self) -> ViewRef<TasksSelectView> {
        self.get_main_dialog()
            .find_name(TasksSelectView::view_name().as_str())
            .unwrap()
    }
}

impl Default for TasksView {
    /// Creates Dialog with LinearLayout inside
    /// LinearLayout consists of the view for display tasks
    /// and the edit view to allow search throught tasks.
    fn default() -> Self {
        let tasks_view_edit_view = TasksSearchView::default();
        let inner_tasks_select_view = TasksSelectView::default()
            .with_name(TasksSelectView::view_name())
            .scrollable();
        Self {
            inner_view: Dialog::new()
                .title("Choose issue")
                .padding_lrtb(1, 1, 1, 1)
                .content(
                    LinearLayout::vertical()
                        // .child(search_task_view)
                        .child(tasks_view_edit_view)
                        .child(inner_tasks_select_view),
                )
                .with_name(Self::main_dialog_name()),
        }
    }
}

impl JiraViewWithName for TasksView {
    /// Returns name of the TasksView.
    fn view_name() -> String {
        "TasksView".into()
    }

    /// Returns instance of the TasksView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for TasksView {
    /// Returns name of the main Dialog in TasksView.
    fn main_dialog_name() -> String {
        "TasksDialogName".into()
    }

    /// Returns instance of the main Dialog in TasksView.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl ChangeJiraView for TasksView {
    /// Updates SelectView in TasksView with data from JiraData.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        let mut tasks_select_view: ViewRef<TasksSelectView> =
            self.get_select_view();

        let jira_data: Arc<RwLock<JiraData>> =
            cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();

        let jira_guard = jira_data_clone.read().unwrap();

        match jira_guard.get_selected_project().unwrap().tasks_names() {
            Some(tasks_names) => {
                tasks_select_view.inner_view.clear();
                tasks_select_view.inner_view.add_all_str(tasks_names);
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
        self.get_select_view().inner_view.add_all_str(content);
    }
}

#[derive(ViewWrapper)]
struct TasksSearchView {
    inner_view: NamedView<Dialog>,
}

impl JiraViewWithName for TasksSearchView {
    /// Returns name of the TasksViewSearchView.
    fn view_name() -> String {
        "TasksViewSearchView".into()
    }

    /// Returns instance of the TasksViewSearchView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for TasksSearchView {
    /// Returns name of the main Dialog in TasksViewSearchView.
    fn main_dialog_name() -> String {
        "TasksViewSearchViewDialog".into()
    }

    /// Returns instance of the main Dialog in TasksViewSearchView.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(&Self::main_dialog_name()).unwrap()
    }
}

impl Default for TasksSearchView {
    fn default() -> Self {
        let task_search_view = EditView::new()
            .on_edit(|cursive: &mut Cursive, task_subname: &str, _: usize| {
                Self::on_edit_task_search(cursive, task_subname);
            })
            .on_submit(Self::on_submit_task_search);
        Self {
            inner_view: Dialog::new()
                .title("Search for tasks")
                .content(task_search_view)
                .with_name(Self::main_dialog_name()),
        }
    }
}

impl TasksSearchView {
    fn on_submit_task_search(cursive: &mut Cursive, issue_key: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();

        let mut is_issue_exist: bool = false;

        {
            let mut jira_data_guard = jira_data.write().unwrap();
            let selected_project_key =
                jira_data_guard.get_selected_project().unwrap().key.clone();

            if jira_data_guard.set_selected_task(issue_key).is_none() {
                return;
            }
            let task = JiraIssue::new(
                jira_data_guard.client.clone(),
                format!("{}-{}", selected_project_key, issue_key).as_str(),
            );

            if let Ok(mut task) = task {
                task.add_transitions(jira_data_guard.client.clone());
                jira_data_guard.add_new_task(task);
                is_issue_exist = true;
            }
        };

        if is_issue_exist {
            InfoView::get_view(cursive).update_view_content(cursive);
            Self::on_edit_task_search(cursive, issue_key);
        } else {
            cursive.add_layer(FailedAttemptView::new(
                format!("Can't find task with key: {}", issue_key).as_str(),
            ))
        }
    }

    fn on_edit_task_search(cursive: &mut Cursive, task_subname: &str) {
        let jira_data: Arc<RwLock<JiraData>> =
            cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();

        let mut tasks_select_view: ViewRef<TasksSelectView> =
            TasksSelectView::get_view(cursive);

        let jira_data_guard = jira_data_clone.read().unwrap();
        let fit_tasks = jira_data_guard.find_task_by_subname(
            task_subname,
            &jira_data_guard.selected_project,
        );

        if fit_tasks.is_some() {
            let unwrap_tasks = fit_tasks.as_ref().unwrap();
            if unwrap_tasks.is_empty() {
                tasks_select_view.inner_view.clear();
            } else {
                tasks_select_view.inner_view.clear();
                for task in unwrap_tasks {
                    tasks_select_view.inner_view.add_item_str(format!(
                        "{} -- {}",
                        task.key, task.summary
                    ));
                }
            }
        };
        cursive.set_user_data(jira_data);
    }
}

#[derive(ViewWrapper)]
pub struct TasksSelectView {
    pub inner_view: SelectView,
}

impl JiraViewWithName for TasksSelectView {
    /// Returns name of the TasksSelectView.
    fn view_name() -> String {
        "TasksSelectViewName".into()
    }

    /// Returns instance of the TasksSelectView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl Default for TasksSelectView {
    fn default() -> Self {
        let tasks_select_view = SelectView::<String>::new()
            .align(INNER_LEFT_TOP_VIEW_ALIGN)
            .on_submit(Self::on_submit_tasks_select_view);
        Self {
            inner_view: tasks_select_view,
        }
    }
}

impl TasksSelectView {
    fn on_submit_tasks_select_view(cursive: &mut Cursive, task_name: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();

        {
            let mut jira_data_guard = jira_data.write().unwrap();

            jira_data_guard.set_selected_task(task_name);
            let client = jira_data_guard.client.clone();

            let task = jira_data_guard.get_mut_selected_task();
            task.add_transitions(client);
        }
        InfoView::get_view(cursive).show_info_on_select(cursive, task_name);
    }
}

#[derive(ViewWrapper)]
pub(crate) struct InfoView {
    inner_view: NamedView<Dialog>,
}

impl Default for InfoView {
    fn default() -> Self {
        Self::new(IssueBaseInfo::default())
    }
}

impl JiraViewWithName for InfoView {
    /// Returns name of the InfoView.
    fn view_name() -> String {
        "InfoView".into()
    }

    /// Returns the instance of the InfoView.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl JiraWithDialogView for InfoView {
    /// Returns name of the main dialog.
    fn main_dialog_name() -> String {
        "InfoViewDialog".into()
    }

    /// Returns the instance of the main Dialog.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog> {
        self.find_name(Self::main_dialog_name().as_str()).unwrap()
    }
}

impl ChangeJiraView for InfoView {
    /// Updates view with task information.
    ///
    /// In fact, just recreate InfoView without data and
    /// add it to InfoLayout.
    fn update_view_content(&mut self, cursive: &mut Cursive) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();
        let jira_data_guard = jira_data.write().unwrap();
        let task = {
            let selected_task = &jira_data_guard.selected_task;
            jira_data_guard
                .get_selected_project()
                .unwrap()
                .get_task(selected_task)
        };
        let mut assignee_name: &str = "Unassigned";
        if let Some(assignee) = &task.assignee {
            assignee_name = assignee.name.as_str();
        }
        let issue_base_info = IssueBaseInfo::new(
            IssueBaseInfoField::new("Summary", &task.summary),
            IssueBaseInfoField::new("Description", &task.description),
            IssueBaseInfoField::new("Issue", &task.key),
            IssueBaseInfoField::new("Status", &task.status.name),
            IssueBaseInfoField::new("Assignee", assignee_name),
        );
        self.get_main_dialog()
            .set_content(Self::make_inner_view(issue_base_info));
    }
}

impl InfoView {
    fn new(issue_base_info: IssueBaseInfo) -> Self {
        let dialog = Dialog::new()
            .title("Task information")
            .content(Self::make_inner_view(issue_base_info))
            .with_name(Self::main_dialog_name());

        Self { inner_view: dialog }
    }

    fn make_inner_view(issue_base_info: IssueBaseInfo) -> LinearLayout {
        let top_inner_view_layout = LinearLayout::horizontal()
            .child(
                InfoView::make_summary_dialog(&issue_base_info).full_width(),
            )
            .child(InfoView::make_issue_main_info_dialog(&issue_base_info));

        LinearLayout::vertical()
            .child(top_inner_view_layout)
            .child(DummyView)
            .child(InfoView::make_description_dialog(issue_base_info))
    }

    fn make_summary_dialog(issue_base_info: &IssueBaseInfo) -> Dialog {
        let title: String;
        if issue_base_info.task_key.inner_value.is_empty() {
            title = "No task selected".into();
        } else {
            title = "Issue title".into()
        }
        Dialog::new().title(title).content(
            cursive_markup::MarkupView::html(
                issue_base_info.summary.inner_value,
            )
            .with_name("summary_task_view"),
        )
    }

    fn make_issue_main_info_dialog(issue_base_info: &IssueBaseInfo) -> Dialog {
        let main_info_dialog = Dialog::new().title("Main issue information");
        if issue_base_info.task_key.inner_value.is_empty() {
            main_info_dialog
        } else {
            let main_info_inner_layout = LinearLayout::vertical()
                .child(TextView::new(format!(
                    "{} - {}",
                    issue_base_info.task_key.display_name,
                    issue_base_info.task_key.inner_value
                )))
                .child(TextView::new(format!(
                    "{} - {}",
                    issue_base_info.task_status_name.display_name,
                    issue_base_info.task_status_name.inner_value
                )))
                .child(TextView::new(format!(
                    "{} - {}",
                    issue_base_info.issue_assignee.display_name,
                    issue_base_info.issue_assignee.inner_value,
                )));

            main_info_dialog.content(main_info_inner_layout)
        }
    }

    fn make_description_dialog(issue_base_info: IssueBaseInfo) -> Dialog {
        Dialog::new()
            .title("Description")
            .padding_lrtb(1, 1, 1, 1)
            .content(ScrollView::new(
                cursive_markup::MarkupView::html(
                    issue_base_info.description.inner_value,
                )
                .with_name("description_task_view"),
            ))
    }

    /// Shows task information in InfoView.
    fn show_info_on_select(&mut self, cursive: &mut Cursive, task_name: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> =
            cursive.user_data().unwrap();
        let task_key: Vec<&str> = task_name.split(" -- ").collect();

        let jira_data_guard = jira_data.read().unwrap();
        let task = jira_data_guard
            .get_project(jira_data_guard.selected_project.clone().as_str())
            .unwrap()
            .get_task(task_key[0]);

        let mut assignee_name: &str = "Unassigned";
        if let Some(assignee) = &task.assignee {
            assignee_name = assignee.name.as_str();
        }
        let issue_base_info = IssueBaseInfo::new(
            IssueBaseInfoField::new("Summary", &task.summary),
            IssueBaseInfoField::new("Description", &task.description),
            IssueBaseInfoField::new("Issue", &task.key),
            IssueBaseInfoField::new("Status", &task.status.name),
            IssueBaseInfoField::new("Assignee", assignee_name),
        );
        self.get_main_dialog()
            .set_content(Self::make_inner_view(issue_base_info));
    }
}
