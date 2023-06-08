use crate::jira::{
    common::views::JiraView,
    projects::views::ProjectsView,
    tasks::views::{InfoView, TasksView},
};
use cursive::{
    view::{Nameable, Resizable, ViewWrapper},
    views::{DummyView, LinearLayout, NamedView},
    View,
};
use rusji_derive::ViewWrapper;

#[derive(ViewWrapper)]
pub(crate) struct TasksProjectsLayout {
    inner_layout: LinearLayout,
}

impl Default for TasksProjectsLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(
                    ProjectsView::default()
                        .with_name(ProjectsView::view_name())
                        .full_height(),
                )
                .child(DummyView)
                .child(
                    TasksView::default()
                        .with_name(TasksView::view_name())
                        .full_height(),
                ),
        }
    }
}

#[derive(ViewWrapper)]
pub(crate) struct InfoLayout {
    inner_layout: NamedView<LinearLayout>,
}

impl Default for InfoLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(InfoView::default().with_name(InfoView::view_name()))
                .with_name(Self::inner_layout_name()),
        }
    }
}

impl InfoLayout {
    pub fn layout_name() -> String {
        String::from("InfoLayout")
    }

    pub fn inner_layout_name() -> String {
        String::from("InnerInfoLayout")
    }
}
