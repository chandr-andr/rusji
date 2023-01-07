use cursive::Cursive;
use super::{
    constance::{TASKS_SEARCH_VIEW_NAME},
    views::ProjectsView,
};

pub(crate) fn add_global_callbacks(cursive: &mut Cursive) {
    cursive.add_global_callback('p', |cursive| {
        cursive.focus_name(&ProjectsView::search_view_name()).unwrap();
    });
    cursive.add_global_callback('t', |cursive| {
        cursive.focus_name(TASKS_SEARCH_VIEW_NAME).unwrap();
    });
}
