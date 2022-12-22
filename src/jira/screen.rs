use std::cell::RefCell;

use cursive::{
    views::{
        LinearLayout,
        SelectView,
        ResizedView,
        ScrollView,
        ViewRef,
        DummyView,
        NamedView,
        Dialog,
        EditView,
        TextView,
    },
    view::{Resizable, Nameable},
    Cursive,
};
use cursive::align;
use cursive_markup;
use crate::Config;
use super::{jira_data::JiraData};

const INNER_CENTER_TOP_VIEW_ALIGN: align::Align = align::Align {
    h: align::HAlign::Center,
    v: align::VAlign::Top,
};

const INNER_LEFT_TOP_VIEW_ALIGN: align::Align = align::Align {
    h: align::HAlign::Left,
    v: align::VAlign::Top,
};

struct CursiveJiraData<'a> {
    config: Config,
    jira_data: JiraData<'a>,
    company_name: String,
    selected_project: String,
}

impl<'a> CursiveJiraData<'a> {
    fn new(config: Config, company_name: &str, jira_data: JiraData<'a>) -> Self {
        CursiveJiraData {
            jira_data: jira_data,
            config: config,
            company_name: company_name.to_string(),
            selected_project: String::default(),
        }
    }
}

pub fn make_jira_screen(cursive: &mut Cursive, company_name: &str) {
    let config = Config::new().unwrap();
    let jira = config.get_jira_by_company(company_name).unwrap();
    let mut jira_data = JiraData::new(jira.get_url());
    jira_data.update_projects(jira.get_encoded_creds()).unwrap();

    let screen_size = cursive.screen_size();
    let side_width = screen_size.x * 2 / 7;
    let center_width = screen_size.x * 3 / 7;

    let mut main_layer = make_main_layer();

    let tasks_projects_layer = make_tasks_projects_layer(&jira_data);
    let info_layer = make_info_layer();
    let actions_something_layer = make_actions_something_layer();

    main_layer.add_child(tasks_projects_layer.min_width(side_width));
    main_layer.add_child(info_layer.min_width(center_width));
    main_layer.add_child(actions_something_layer.min_width(side_width));

    cursive.add_layer(main_layer);

    let c_jira_data = CursiveJiraData::new(
        config,
        company_name,
        jira_data,
    );
    cursive.set_user_data(c_jira_data);
}

fn make_main_layer() -> LinearLayout {
    LinearLayout::horizontal()
}

fn make_tasks_projects_layer(jira_data: &JiraData) -> LinearLayout {
    let mut tasks_projects_layout = LinearLayout::vertical();

    tasks_projects_layout.add_child(make_projects_view(jira_data));
    tasks_projects_layout.add_child(DummyView);
    tasks_projects_layout.add_child(make_tasks_view());

    tasks_projects_layout
}

fn make_info_layer() -> NamedView<LinearLayout> {
    let mut info_layout = LinearLayout::vertical();

    info_layout.add_child(make_info_view("Choose task firstly", ""));

    info_layout.with_name("info_layout")
}

fn make_actions_something_layer() -> LinearLayout {
    let mut info_layout = LinearLayout::vertical();
    info_layout.add_child(make_actions_view());
    info_layout.add_child(DummyView);
    info_layout.add_child(make_something_view());
    info_layout
}

fn make_projects_view(jira_data: &JiraData) -> Dialog {
    let search_view = EditView::new().on_submit(on_enter_search_project);

    let search_project_dialog = Dialog::new()
        .title("Search project by name")
        .content(search_view);

    let mut inner_projects_view = SelectView::<String>::new()
        .align(INNER_CENTER_TOP_VIEW_ALIGN)
        .on_submit(show_tasks);

    let projects_names = jira_data.get_projects_names();
    inner_projects_view.add_all_str(
        projects_names,
    );

    let projects_scroll_view = ScrollView::new(
        inner_projects_view
            .with_name("projects_view"));

    let inner_projects_layout = LinearLayout::vertical()
        .child(search_project_dialog)
        .child(DummyView)
        .child(projects_scroll_view);

    Dialog::new()
        .title("Choose project")
        .padding_lrtb(1, 1, 1, 1)
        .content(inner_projects_layout)
}

fn make_tasks_view() -> Dialog {
    let inner_tasks_view = SelectView::<String>::new()
        .align(INNER_LEFT_TOP_VIEW_ALIGN)
        .on_submit(show_info_on_select);

    Dialog::new()
        .title("Choose issue")
        .padding_lrtb(1, 1, 1, 1)
        .content(
        ScrollView::new(
            inner_tasks_view
                .with_name("tasks_view")
            ).full_height()
        )
}

fn make_info_view(summary: &str, description: &str) -> Dialog {
    let summary_inner_info_view = cursive_markup
        ::MarkupView
        ::html(summary)
        .with_name("summary_task_view");
    let summary_dialog = Dialog::new()
        .title("Задача")
        .padding_lrtb(1, 1, 1, 1)
        .content(summary_inner_info_view);

    let description_inner_info_view = cursive_markup
        ::MarkupView
        ::html(description)
        .with_name("description_task_view");
    let description_dialog = Dialog::new()
        .title("Описание")
        .padding_lrtb(1, 1, 1, 1)
        .content(ScrollView::new(description_inner_info_view));

    let info_layout = LinearLayout::vertical()
        .child(summary_dialog)
        .child(DummyView)
        .child(description_dialog);

    Dialog::new()
        .title("Main information about task")
        .content(
            ScrollView::new(info_layout.with_name("info_view"))
        )
}

fn make_actions_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let mut inner_actions_view = SelectView::<String>::new()
        .align(INNER_CENTER_TOP_VIEW_ALIGN);

    inner_actions_view.add_all_str(vec!["Action1", "Action2", "Action3", "Action4"]);

    ScrollView::new(inner_actions_view.with_name("actions_view")).full_height()
}

fn make_something_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let inner_something_view = SelectView::<String>::new()
        .align(INNER_CENTER_TOP_VIEW_ALIGN)
        .with_name("tasks_view");
    ScrollView::new(inner_something_view).full_height()
}

fn show_tasks(cursive: &mut Cursive, project_name: &str) {
    pop_front_layout(cursive, "project_search");
    let mut tasks_view: ViewRef<SelectView> = cursive.find_name("tasks_view").unwrap();
    let c_jira_data: &mut CursiveJiraData = cursive.user_data().unwrap();
    c_jira_data.selected_project = project_name.to_string();

    let encoded_creds = c_jira_data
        .config
        .get_jira_by_company(&c_jira_data.company_name)
        .unwrap()
        .get_encoded_creds();

    c_jira_data
        .jira_data
        .update_tasks(project_name, encoded_creds)
        .unwrap();

    let project_tasks = c_jira_data
        .jira_data
        .get_tasks_names_by_project(project_name)
        .unwrap();

    tasks_view.clear();
    tasks_view.add_all_str(project_tasks);

    cursive.focus_name("tasks_view").unwrap();
}

fn show_info_on_select(cursive: &mut Cursive, task_name: &str) {
    let mut info_layout: ViewRef<LinearLayout> = cursive.find_name("info_layout").unwrap();
    let c_jira_data: &CursiveJiraData = cursive.user_data().unwrap();
    let task_key: Vec<&str> = task_name.split(" -- ").collect();

    let (summary, description) = c_jira_data
        .jira_data
        .get_task_description(&c_jira_data.selected_project, task_key[0]);

    let new_info_view = make_info_view(summary, description);
    info_layout.clear();
    info_layout.add_child(new_info_view);
}

fn on_enter_search_project(cursive: &mut Cursive, project_subname: &str) {
    let c_jira_data: &CursiveJiraData = cursive.user_data().unwrap();
    let fit_projects = c_jira_data
        .jira_data
        .find_project_by_subname(project_subname);


    let fit_projects_select_view = SelectView::<String>::new()
        .align(INNER_CENTER_TOP_VIEW_ALIGN)
        .on_submit(show_tasks);

    let fit_projects_dialog = Dialog::new()
        .title("Select project")
        .content(fit_projects_select_view.with_all_str(fit_projects))
        .with_name("project_search");

    cursive.add_layer(fit_projects_dialog);
}

fn pop_front_layout(cursive: &mut Cursive, layer_name: &str) {
    let to_pop_layer: Option<ViewRef<Dialog>> = cursive.find_name(layer_name);
    if let Some(_) = to_pop_layer {
        cursive.pop_layer();
        return ();
    }
}