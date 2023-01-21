use super::{
    global_callbacks::add_global_callbacks,
    jira_data::{CursiveJiraData, JiraData},
    layouts::{ActionsLayout, InfoLayout, TasksProjectsLayout},
};
use crate::jira::{common::views::JiraView, projects::views::ProjectsView};

use crate::Config;
use cursive::{
    view::{Nameable, Resizable},
    views::LinearLayout,
    Cursive,
};

pub fn make_jira_screen(cursive: &mut Cursive, company_name: &str) {
    add_global_callbacks(cursive);
    let config = Config::new().unwrap();
    let jira = config.get_jira_by_company(company_name).unwrap();
    let jira_data = JiraData::new(jira.get_url(), jira.get_encoded_creds());
    let cursive_data = CursiveJiraData::new(jira_data);
    cursive.set_user_data(cursive_data);

    let screen_size = cursive.screen_size();
    let side_width = screen_size.x * 2 / 7;
    let center_width = screen_size.x * 3 / 7;

    let mut main_layer = LinearLayout::horizontal();

    let tasks_projects_layer = TasksProjectsLayout::default();
    let info_layer = InfoLayout::default().with_name(InfoLayout::layout_name());
    let actions_something_layer = ActionsLayout::default();

    main_layer.add_child(tasks_projects_layer.min_width(side_width));
    main_layer.add_child(info_layer.min_width(center_width));
    main_layer.add_child(actions_something_layer.min_width(side_width));

    cursive.add_layer(main_layer);

    ProjectsView::get_view(cursive).update_view_content(cursive);
}
