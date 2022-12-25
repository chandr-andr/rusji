use cursive::Cursive;
use super::constance::PROJECTS_SEARCH_VIEW_NAME;

pub(crate) fn add_global_callbacks(cursive: &mut Cursive) {
    cursive.add_global_callback('p', go_to_project_search_callback)
}

fn go_to_project_search_callback(cursive: &mut Cursive) {
    cursive.focus_name(PROJECTS_SEARCH_VIEW_NAME).unwrap();
}