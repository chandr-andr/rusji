use std::sync::{Arc, RwLock};

use cursive::{
    view::{Nameable, Resizable, ViewWrapper},
    views::{Dialog, EditView, NamedView, ResizedView},
    Cursive, View,
};

use rusji_derive::ViewWrapper;

use crate::{
    jira::{
        common::views::{ButtonView, JiraViewWithName, ToggleableView},
        utils::{helpers::calculate_view_size, views::FailedAttemptView},
    },
    jira_data::JiraData,
};

/// Main view for changing story points.
#[derive(ViewWrapper)]
pub struct ChangeSPView {
    inner_view: NamedView<ResizedView<Dialog>>,
}

impl ToggleableView for ChangeSPView {}

impl ButtonView for ChangeSPView {
    fn inner_view(self) -> NamedView<ResizedView<Dialog>> {
        self.inner_view
    }
}

impl JiraViewWithName for ChangeSPView {
    /// Returns name of the `ChangeSPView`.
    ///
    /// It will used for `.with_name()` method.
    fn view_name() -> String {
        "ChangeSPView".into()
    }

    /// Returns instance of `ChangeSPView`
    fn get_view(
        cursive: &mut cursive::Cursive,
    ) -> cursive::views::ViewRef<Self> {
        cursive.find_name(Self::view_name().as_str()).unwrap()
    }
}

impl ChangeSPView {
    pub fn new(cursive: &mut Cursive) -> Self {
        Self::toggle_on_view(cursive);
        let change_sp_view = Dialog::new()
            .title("Change story points")
            .content(ChangeSPEditView::new())
            .fixed_size(calculate_view_size(cursive, 2, 7))
            .with_name(Self::view_name());
        Self {
            inner_view: change_sp_view,
        }
    }
}

#[derive(ViewWrapper)]
struct ChangeSPEditView {
    inner_view: ResizedView<Dialog>,
}

impl ChangeSPEditView {
    pub fn new() -> Self {
        let change_sp_edit_view = Dialog::new()
            .title("Enter new story points amount")
            .content(
                EditView::new()
                    .on_submit(Self::on_submit_enter_story_points)
                    .full_width()
                    .full_height(),
            )
            .full_screen();
        Self {
            inner_view: change_sp_edit_view,
        }
    }

    fn on_submit_enter_story_points(
        cursive: &mut Cursive,
        story_points: &str,
    ) {
        let story_point_in_usize = story_points.parse::<usize>();
        if let Err(_) = story_point_in_usize {
            cursive.pop_layer();
            let bad_story_points_type_view = FailedAttemptView::new(
                "You must specify story points as an integer",
            );
            cursive.add_layer(bad_story_points_type_view);
            return;
        }

        let (request_client, issue_key) = {
            let jira_data: &mut Arc<RwLock<JiraData>> =
                cursive.user_data().unwrap();
            let jira_data_guard = jira_data.read().unwrap();
            let request_client = jira_data_guard.client.clone();
            let selected_issue_key =
                jira_data_guard.get_selected_task().key.clone();

            (request_client, selected_issue_key)
        };

        let request_result =
            request_client.read().unwrap().update_issue_story_points(
                story_point_in_usize.unwrap(),
                issue_key.as_str(),
            );

        if let Err(_) = request_result {
            cursive.add_layer(FailedAttemptView::new(
                format!(
                    "Can't change story points for some reason. Try again",
                )
                .as_str(),
            ));
            cursive.pop_layer();
            return;
        }

        cursive.pop_layer();
    }
}
