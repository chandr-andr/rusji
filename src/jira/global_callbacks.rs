use cursive::Cursive;
use super::constance::{PROJECTS_SEARCH_VIEW_NAME, TASKS_SEARCH_VIEW_NAME};

pub(crate) fn add_global_callbacks(cursive: &mut Cursive) {
    cursive.add_global_callback('p', |cursive| {
        cursive.focus_name(PROJECTS_SEARCH_VIEW_NAME).unwrap();
    });
    cursive.add_global_callback('t', |cursive| {
        cursive.focus_name(TASKS_SEARCH_VIEW_NAME).unwrap();
    });
}
