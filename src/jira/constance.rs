use cursive::align;

pub(crate) const TASKS_SEARCH_VIEW_NAME: &str = "task_search";
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