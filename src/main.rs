mod cli;
mod startup;
mod utils;
mod constance;
mod config;
mod actions;
mod jira;
pub use crate::cli::*;
pub use crate::startup::*;
pub use crate::actions::*;
pub use crate::config::*;
pub use crate::jira::*;

fn main() {
    start_screen();
    // let is_startup_success = startup();
    // parse_args();
}
