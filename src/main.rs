mod cli;
mod startup;
mod utils;
mod constance;
mod config;
mod screen;
mod jira;
mod theme;
pub use crate::cli::*;
pub use crate::startup::*;
pub use crate::screen::*;
pub use crate::config::*;
pub use crate::jira::*;

fn main() {
    let config = Config::new().unwrap();
    start_screen(config);
}
