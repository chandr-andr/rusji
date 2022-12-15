use std::collections::HashMap;

use cursive::Cursive;
use cursive::view::{Resizable, Nameable};
use cursive::views::{
    LinearLayout,
    SelectView, DummyView, Dialog, EditView, TextView,
};
use crate::Config;
use crate::jira::screen::make_jira_screen;

struct CursiveUserData<'a> {
    config: Config,
    selected_company: String,
    to_find_names: Vec<&'a str>,
}

impl<'a> CursiveUserData<'static> {
    fn new(config: Config) -> Self {
        CursiveUserData {
            config: config,
            selected_company: String::default(),
            to_find_names: Vec::default(),
        }
    }

    fn refresh_config(&mut self) {
        self.config = Config::new().unwrap();
    }
}

pub fn start_screen(config: Config) {
    let mut cursive = cursive::default();

    let c_user_data = CursiveUserData::new(config);
    cursive.set_user_data(c_user_data);

    let c_user_data: CursiveUserData = cursive.take_user_data().unwrap();
    let mut start_screen_layout = LinearLayout::vertical();

    let mut exist_companies_select_view = SelectView::<String>::new()
        .on_submit(on_select_project);
    exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut add_new_company_select_view = SelectView::<String>::new()
        .on_submit(add_new_company_screen);
    add_new_company_select_view.add_all_str(vec!["Add new company"]);

    start_screen_layout.add_child(exist_companies_select_view);
    start_screen_layout.add_child(DummyView);
    start_screen_layout.add_child(add_new_company_select_view);

    cursive.set_user_data(c_user_data);

    let start_screen_dialog = Dialog::new()
        .title("Select company or add one")
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
        let edit_view = EditView::new()
            .with_name(*view_name)
            .min_width(20);

        edit_layout.add_child(TextView::new(*view_name));
        edit_layout.add_child(edit_view);
        edit_layout.add_child(DummyView);
    }

    let mut c_user_data: CursiveUserData = cursive.take_user_data().unwrap();
    c_user_data.to_find_names = views_names;
    cursive.set_user_data(c_user_data);

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
            .call_on_name(view_name, |view: &mut EditView| {
                view.get_content()
            }).unwrap();

        // TODO: check if all fields were filled

        input_company_data.insert(view_name, view_info.to_string());
    }

    let is_add_success = c_user_data.config.add_new_company(
        input_company_data.get("Jira URL").unwrap(),
        input_company_data.get("Company name").unwrap(),
        input_company_data.get("Jira username/login").unwrap(),
        input_company_data.get("Jira Password").unwrap(),
    );

    match is_add_success {
        Ok(_) => {
            c_user_data.refresh_config();
            cursive.set_user_data(c_user_data);
            succes_company_add(cursive)
        },
        Err(err) => println!("{}", err),
    }
    println!("{:?}", input_company_data);
}

fn succes_company_add(cursive: &mut Cursive) {
    cursive.pop_layer();
    cursive.add_layer(
        Dialog::new()
            .title("Success!")
            .content(TextView::new("Company added successfully!"))
            .padding_lrtb(1, 1, 1, 1)
            .button("OK", |cursive: &mut Cursive| {
                set_start_screen(cursive)
            })
    )
}

// Creates start screen with availability to select or add company.
pub fn set_start_screen(cursive: &mut Cursive) {
    let c_user_data: CursiveUserData = cursive.take_user_data().unwrap();
    let mut start_screen_layout = LinearLayout::vertical();

    let mut exist_companies_select_view = SelectView::<String>::new();
    exist_companies_select_view.add_all_str(c_user_data.config.companies_names());

    let mut add_new_company_select_view = SelectView::<String>::new()
        .on_submit(add_new_company_screen);
    add_new_company_select_view.add_all_str(vec!["Add new company"]);

    start_screen_layout.add_child(exist_companies_select_view);
    start_screen_layout.add_child(DummyView);
    start_screen_layout.add_child(add_new_company_select_view);

    cursive.set_user_data(c_user_data);

    let start_screen_dialog = Dialog::new()
        .title("Select company or add one")
        .content(start_screen_layout)
        .padding_lrtb(1, 1, 1, 1);

    cursive.pop_layer();
    cursive.add_layer(start_screen_dialog.min_width(25).max_width(50));
}

fn on_select_project(cursive: &mut Cursive, project: &str) {
    cursive.pop_layer();
    make_jira_screen(cursive);
}