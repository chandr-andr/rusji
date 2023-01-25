use std::sync::{Arc, RwLock};

use cursive::View;
use cursive::{
    view::{Finder, Nameable, Resizable, Scrollable, ViewWrapper},
    views::{
        Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView, SelectView, TextView,
        ViewRef,
    },
    Cursive,
};

use crate::jira::common::views::JiraView;
use crate::jira::constance::{
    ACTIONS_SELECT_VIEW_NAME, INNER_CENTER_TOP_VIEW_ALIGN, INNER_LEFT_TOP_VIEW_ALIGN,
};
use crate::jira_data::JiraData;

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
                            InfoView::get_view(cursive).make_http_search(cursive, task_key)
                        })
                        .with_name(Self::search_view_name()),
                );

            Dialog::new().title("Search task by name").content(layout)
        };

        let inner_tasks_view = SelectView::<String>::new()
            .align(INNER_LEFT_TOP_VIEW_ALIGN)
            .on_submit(|cursive: &mut Cursive, task_name: &str| {
                InfoView::get_view(cursive).show_info_on_select(cursive, task_name);
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
        self.update_tasks(cursive);
    }

    /// Sets new content for SelectView in TasksView from passed `content`.
    fn set_view_content(&mut self, content: Vec<&str>) {
        let mut select_view = self.get_select_view();
        select_view.clear();
        select_view.add_all_str(content);
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

    /// Returns instance of the EditView in TasksView.
    pub fn get_search_view(&mut self) -> ViewRef<EditView> {
        self.get_main_dialog()
            .find_name(Self::search_view_name().as_str())
            .unwrap()
    }

    /// Updates tasks.
    ///
    /// Uses JiraData instance to get updates tasks.
    ///
    /// After updating focus on TasksView.
    fn update_tasks(&mut self, cursive: &mut Cursive) {
        let mut tasks_select_view: ViewRef<SelectView> = self.get_select_view();

        let jira_data: Arc<RwLock<JiraData>> = cursive.take_user_data().unwrap();
        let selected_project = jira_data.read().unwrap().selected_project.clone();

        let project_tasks = jira_data
            .write()
            .unwrap()
            .update_return_tasks(selected_project.as_str());
        {
            tasks_select_view.clear();
            tasks_select_view.add_all_str(project_tasks);
            tasks_select_view.sort();
        }

        cursive.focus_name(&TasksView::view_name()).unwrap();
        cursive.set_user_data(jira_data);
    }

    /// Tries to find task to display it.
    fn on_enter_task_search(&mut self, cursive: &mut Cursive, task_subname: &str) {
        let jira_data: Arc<RwLock<JiraData>> = cursive.take_user_data().unwrap();
        let jira_data_clone = jira_data.clone();
        let mut tasks_select_view: ViewRef<SelectView> = self.get_select_view();

        let jira_data_guard = jira_data_clone.read().unwrap();
        let fit_tasks = jira_data_guard.find_task_by_subname(
            task_subname,
            jira_data_guard.selected_project.clone().as_str(),
        );

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
    fn update_view_content(&mut self, _: &mut Cursive) {
        self.get_main_dialog()
            .set_content(Self::make_inner_view("", ""));
    }

    /// Sets new content of the InfoView.
    ///
    /// In fact, we completely clear the layout and
    /// create a completely new view, which we add
    /// new view.
    fn set_view_content(&mut self, content: Vec<&str>) {
        self.get_main_dialog()
            .set_content(Self::make_inner_view(content[0], content[1]));
    }

    /// Does the same as `set_view_content` method.
    fn add_content_to_view(&mut self, content: Vec<&str>) {
        self.set_view_content(content)
    }
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
            .title("Задача")
            .content(cursive_markup::MarkupView::html(summary).with_name("summary_task_view"))
    }

    fn make_description_dialog(description: &str) -> Dialog {
        Dialog::new()
            .title("Описание")
            .padding_lrtb(1, 1, 1, 1)
            .content(ScrollView::new(
                cursive_markup::MarkupView::html(description).with_name("description_task_view"),
            ))
    }

    /// Shows task information in InfoView.
    fn show_info_on_select(&mut self, cursive: &mut Cursive, task_name: &str) {
        let jira_data: &mut Arc<RwLock<JiraData>> = cursive.user_data().unwrap();
        let selected_project = jira_data.read().unwrap().selected_project.clone();
        let task_key: Vec<&str> = task_name.split(" -- ").collect();

        let jira_data_guard = jira_data.read().unwrap();
        let task = jira_data_guard
            .get_project(&selected_project)
            .get_task(task_key[0]);

        self.set_view_content(vec![&task.summary, &task.description]);
    }

    /// Makes API call to try find task that not in app.
    ///
    /// If task isn't found display new view with error text.
    fn make_http_search(&mut self, cursive: &mut Cursive, task_key: &str) {
        let mut jira_data: JiraData = cursive.take_user_data().unwrap();
        match jira_data.get_new_task(task_key) {
            Ok((summary, desc)) => {
                self.set_view_content(vec![summary.as_str(), desc.as_str()]);
                TasksView::get_view(cursive);
            }
            Err(_) => cursive.add_layer(Dialog::new().title("Task not found").button(
                "Ok",
                |cursive| {
                    cursive.pop_layer();
                },
            )),
        }
    }
}

pub struct ActionsView {
    inner_view: Dialog,
}

impl Default for ActionsView {
    fn default() -> Self {
        let inner_action_view = SelectView::<String>::new()
            .align(INNER_CENTER_TOP_VIEW_ALIGN)
            .with_name(ACTIONS_SELECT_VIEW_NAME);

        Self {
            inner_view: Dialog::new()
                .title("Choose action")
                .content(ScrollView::new(inner_action_view).full_height()),
        }
    }
}

impl ViewWrapper for ActionsView {
    type V = Dialog;

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
