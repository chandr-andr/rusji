use cursive::{
    views::LinearLayout,
    view::Resizable,
    Cursive,
};
use crate::Config;
use super::{
    jira_data::{
        JiraData,
        CursiveJiraData,
    }, layouts::{
        TasksProjectsLayout,
        InfoLayout, ActionsLayout,
    }, global_callbacks::add_global_callbacks,
    views::{ProjectsView, JiraView},
};

pub fn make_jira_screen(cursive: &mut Cursive, company_name: &str) {
    add_global_callbacks(cursive);
    let config = Config::new().unwrap();
    let jira = config.get_jira_by_company(company_name).unwrap();
    let jira_data = JiraData::new(jira.get_url());
    let encoded_creds = config
                .get_jira_by_company(company_name)
                .unwrap()
                .get_encoded_creds()
                .to_string();
    let cursive_data = CursiveJiraData::new(
        encoded_creds,
        jira_data,
    );
    cursive.set_user_data(cursive_data);

    let screen_size = cursive.screen_size();
    let side_width = screen_size.x * 2 / 7;
    let center_width = screen_size.x * 3 / 7;

    let mut main_layer = LinearLayout::horizontal();

    let tasks_projects_layer = TasksProjectsLayout::default();
    let info_layer = InfoLayout::default();
    let actions_something_layer = ActionsLayout::default();

    main_layer.add_child(tasks_projects_layer.min_width(side_width));
    main_layer.add_child(info_layer.min_width(center_width));
    main_layer.add_child(actions_something_layer.min_width(side_width));

    cursive.add_layer(main_layer);

    ProjectsView::get_view(cursive).update_view_content(cursive);
}
