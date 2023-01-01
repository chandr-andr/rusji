use cursive::align;

pub(crate) const PROJECTS_SELECT_VIEW_NAME: &str = "projects_view";
pub(crate) const PROJECTS_SEARCH_VIEW_NAME: &str = "project_search";
pub(crate) const TASKS_SELECT_VIEW_NAME: &str = "tasks_view";
pub(crate) const TASKS_SEARCH_VIEW_NAME: &str = "task_search";
pub(crate) const TASKS_HTTP_SEARCH_VIEW_NAME: &str = "task_http_search";
pub(crate) const ACTIONS_SELECT_VIEW_NAME: &str = "actions_view";
pub(crate) const INFO_LAYOUT_VIEW_NAME: &str = "info_layout";
pub(crate) const INNER_CENTER_TOP_VIEW_ALIGN: align::Align = align::Align {
    h: align::HAlign::Center,
    v: align::VAlign::Top,
};
pub(crate) const INNER_LEFT_TOP_VIEW_ALIGN: align::Align = align::Align {
    h: align::HAlign::Left,
    v: align::VAlign::Top,
};