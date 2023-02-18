use std::collections::HashMap;

use crate::jira::screen::make_jira_screen;
use crate::theme::make_dark_theme;
use crate::Config;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

struct CursiveUserData<'a> {
    config: Config,
    to_find_names: Vec<&'a str>,
}

impl<'a> CursiveUserData<'static> {
    fn new(config: Config) -> Self {
        CursiveUserData {
            config,
            to_find_names: Vec::default(),
        }
    }

    fn refresh_config(&mut self) {
        self.config = Config::new().unwrap();
    }
}

pub fn start_screen(config: Config) {
    let mut cursive = cursive::default();
    cursive.set_theme(make_dark_theme());

    let c_user_data = CursiveUserData::new(config);
    cursive.set_user_data(c_user_data);

    let c_user_data: &mut CursiveUserData = cursive.user_data().unwrap();

    let mut exist_companies_select_view = SelectView::<String>::new().on_submit(on_select_company);
    exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut delete_exist_companies_select_view =
        SelectView::<String>::new().on_submit(delete_company);
    delete_exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut add_new_company_select_view =
        SelectView::<String>::new().on_submit(add_new_company_screen);
    add_new_company_select_view.add_all_str(vec!["Add new company"]);

    let start_screen_layout = LinearLayout::vertical()
        .child(TextView::new("Select a company"))
        .child(DummyView)
        .child(exist_companies_select_view)
        .child(DummyView)
        .child(TextView::new("Maybe you want to add new company?"))
        .child(DummyView)
        .child(add_new_company_select_view)
        .child(DummyView)
        .child(TextView::new("Select a company to delete"))
        .child(DummyView)
        .child(delete_exist_companies_select_view);

    let start_screen_dialog = Dialog::new()
        .title("Select/Add/Delete company")
        .content(start_screen_layout)
        .padding_lrtb(1, 1, 1, 1);

    cursive.add_layer(start_screen_dialog.min_width(25).max_width(50));
    cursive.run();
}

fn add_new_company_screen(cursive: &mut Cursive, _: &str) {
    let views_names = vec![
        "Company name",
        "Jira URL",
        "Jira username/login",
        "Jira Password",
    ];

    let mut edit_layout = LinearLayout::vertical();

    for view_name in &views_names {
        let edit_view = EditView::new().with_name(*view_name).min_width(20);

        edit_layout.add_child(TextView::new(*view_name));
        edit_layout.add_child(edit_view);
        edit_layout.add_child(DummyView);
    }

    let mut c_user_data: &mut CursiveUserData = cursive.user_data().unwrap();
    c_user_data.to_find_names = views_names;

    let add_new_company_dialog = Dialog::new()
        .title("Company name")
        .padding_lrtb(1, 1, 1, 1)
        .content(edit_layout)
        .button("Add", add_new_company)
        .button("Back", set_start_screen);

    cursive.pop_layer();
    cursive.add_layer(add_new_company_dialog)
}

fn add_new_company(cursive: &mut Cursive) {
    let mut c_user_data: CursiveUserData = cursive.take_user_data().unwrap();
    let mut input_company_data: HashMap<&str, String> = HashMap::new();

    for view_name in &c_user_data.to_find_names {
        let view_info = cursive
            .call_on_name(view_name, |view: &mut EditView| view.get_content())
            .unwrap();

        // TODO: check if all fields were filled
        // TODO: check the connection to the jira server

        input_company_data.insert(view_name, view_info.to_string());
    }

    let is_add_success = c_user_data.config.add_new_company(
        input_company_data.get("Jira URL").unwrap(),
        input_company_data.get("Company name").unwrap(),
        input_company_data.get("Jira username/login").unwrap(),
        input_company_data.get("Jira Password").unwrap(),
    );

    if let Ok(_) = is_add_success {
        c_user_data.refresh_config();
        cursive.set_user_data(c_user_data);
        success_dialog(cursive, "Company added successfully!")
    }
}

fn success_dialog(cursive: &mut Cursive, success_text: &str) {
    cursive.pop_layer();
    cursive.add_layer(
        Dialog::new()
            .title("Success!")
            .content(TextView::new(success_text))
            .padding_lrtb(1, 1, 1, 1)
            .button("OK", set_start_screen),
    )
}

// Creates start screen with availability to select or add company.
pub fn set_start_screen(cursive: &mut Cursive) {
    let c_user_data: &mut CursiveUserData = cursive.user_data().unwrap();

    let mut exist_companies_select_view = SelectView::<String>::new().on_submit(on_select_company);
    exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut delete_exist_companies_select_view =
        SelectView::<String>::new().on_submit(delete_company);
    delete_exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut add_new_company_select_view =
        SelectView::<String>::new().on_submit(add_new_company_screen);
    add_new_company_select_view.add_all_str(vec!["Add new company"]);

    let start_screen_layout = LinearLayout::vertical()
        .child(TextView::new("Select a company"))
        .child(DummyView)
        .child(exist_companies_select_view)
        .child(DummyView)
        .child(TextView::new("Maybe you want to add new company?"))
        .child(DummyView)
        .child(add_new_company_select_view)
        .child(DummyView)
        .child(TextView::new("Select a company to delete"))
        .child(DummyView)
        .child(delete_exist_companies_select_view);

    let start_screen_dialog = Dialog::new()
        .title("Select/Add/Delete company")
        .content(start_screen_layout)
        .padding_lrtb(1, 1, 1, 1);

    cursive.pop_layer();
    cursive.add_layer(start_screen_dialog.min_width(25).max_width(50));
}

fn on_select_company(cursive: &mut Cursive, company_name: &str) {
    cursive.pop_layer();
    make_jira_screen(cursive, company_name);
}

fn delete_company(cursive: &mut Cursive, company_name: &str) {
    let c_user_data: &mut CursiveUserData = cursive.user_data().unwrap();
    c_user_data.config.delete_company(company_name).unwrap();
    // TODO: Process Error if needed
    success_dialog(cursive, "Company deleted successfully!")
}
