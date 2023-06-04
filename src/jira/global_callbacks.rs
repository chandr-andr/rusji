use crate::jira::{projects::views::ProjectsView, tasks::views::TasksView};
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
            cursive.pop_layer();
        },
    );

    add_menu_callbacks(cursive);
}

pub(crate) fn add_menu_callbacks(cursive: &mut Cursive) {
    for bottom_button in BottomButtons::new().buttons.into_iter() {
        cursive.add_global_callback(
            bottom_button.keyboard_key,
            bottom_button.action_fn,
        )
    }
}
