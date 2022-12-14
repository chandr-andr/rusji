use cursive::Cursive;
use cursive::views::{
    LinearLayout,
    SelectView,
};
use crate::jira::screen::make_jira_screen;



pub fn start_screen() {
    let mut cursive = cursive::default();

    let mut start_layer = LinearLayout::horizontal();
    let mut start_view = SelectView::<String>::new()
        .on_submit(on_select_project);

    start_view.add_all_str(vec!["Jira", "Exit"]);

    start_layer.add_child(start_view);
    cursive.add_layer(start_layer);

    cursive.run();
}

fn on_select_project(cursive: &mut Cursive, project: &str) {
    cursive.pop_layer();
    if project == "Jira" {
        make_jira_screen(cursive);
    }
    else {
        cursive.quit();
    }
}