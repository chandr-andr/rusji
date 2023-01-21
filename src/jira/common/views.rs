use cursive::{
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
    fn get_main_dialog(&mut self) -> ViewRef<Dialog>;

    /// Updates view content with [`super::jira_data::JiraData`] methods.
    fn update_view_content(&mut self, cursive: &mut Cursive);

    /// Updates view content with passed `content`.
    fn set_view_content(&mut self, content: Vec<&str>);

    /// Extends view content with passed `content`.
    fn add_content_to_view(&mut self, content: Vec<&str>);
}
