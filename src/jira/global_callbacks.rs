use std::sync::{Arc, RwLock};

use crate::{
    jira::{projects::views::ProjectsView, tasks::views::TasksView},
    jira_data::JiraData,
};
use cursive::{
    event::{Event, Key},
    Cursive,
};

use super::bottom_menu::data::BottomButtons;

pub(crate) fn add_global_callbacks(cursive: &mut Cursive) {
    cursive.add_global_callback('p', |cursive| {
        cursive
            .focus_name(&ProjectsView::search_view_name())
            .unwrap();
    });
    cursive.add_global_callback('t', |cursive| {
        cursive.focus_name(&TasksView::search_view_name()).unwrap();
    });

    cursive.add_global_callback(
        Event::Key(Key::Esc),
        |cursive: &mut Cursive| {
            let is_need_to_hide: bool = {
                let jira_data: &mut Arc<RwLock<JiraData>> =
                    cursive.user_data().unwrap();
                let mut jira_data_guard = jira_data.write().unwrap();
                jira_data_guard.activated_views.pop().is_some()
            };
            if is_need_to_hide {
                cursive.pop_layer();
            }
        },
    );

    add_menu_callbacks(cursive);
}

pub(crate) fn add_menu_callbacks(cursive: &mut Cursive) {
    for bottom_button in BottomButtons::new().buttons.into_iter() {
        cursive
            .add_global_callback(bottom_button.event, bottom_button.action_fn)
    }
}
