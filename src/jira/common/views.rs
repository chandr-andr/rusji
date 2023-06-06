use cursive::{
    view::ViewWrapper,
    views::{Dialog, ViewRef},
    Cursive,
};

pub trait JiraView {
    /// Returns name of the view.
    fn view_name() -> String;

    /// Returns instance of class from cursive app.
    fn get_view(cursive: &mut Cursive) -> ViewRef<Self>;

    /// Returns name of the main dialog view.
    fn main_dialog_name() -> String;

    /// Returns instance of main dialog view.   
    /// TODO: Change ViewRef<Dialog> to generic.
    fn get_main_dialog(&mut self) -> ViewRef<Dialog>;

    /// Updates view content from [`super::jira_data::JiraData`] data.
    ///
    /// Default implementation does nothing.
    fn update_view_content(&mut self, _cursive: &mut Cursive) {}

    /// Extends view content with passed `content`.
    ///
    /// Default implementation does nothing.
    fn add_content_to_view(&mut self, _content: Vec<&str>) {}
}

pub trait ActionView: ViewWrapper {
    fn new(cursive: &mut Cursive) -> Self;
}

pub trait ToggleableView: JiraView {
    /// Toggle on view.
    ///
    /// `Toggle on` means add name of this view to
    /// the activated_views in `JiraData` struct.
    ///
    /// It is necessary if we want to have an option
    /// to close first-side views with button.
    fn toggle_on_view(cursive: &mut Cursive);
}
