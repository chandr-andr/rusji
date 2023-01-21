mod cli;
mod config;
mod constance;
mod errors;
mod jira;
mod screen;
mod startup;
mod theme;
mod utils;
pub use crate::cli::*;
pub use crate::config::*;
pub use crate::jira::*;
pub use crate::screen::*;
pub use crate::startup::*;

fn main() {
    let config = Config::new().unwrap();
    start_screen(config);
}
