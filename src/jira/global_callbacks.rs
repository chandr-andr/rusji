use crate::jira::{projects::views::ProjectsView, tasks::views::TasksView};
use cursive::Cursive;

pub(crate) fn add_global_callbacks(cursive: &mut Cursive) {
    cursive.add_global_callback('p', |cursive| {
        cursive
            .focus_name(&ProjectsView::search_view_name())
            .unwrap();
    });
    cursive.add_global_callback('t', |cursive| {
        cursive.focus_name(&TasksView::search_view_name()).unwrap();
    });
}
