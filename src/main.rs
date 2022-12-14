mod cli;
mod startup;
mod utils;
mod constance;
mod config;
mod screen;
mod jira;
pub use crate::cli::*;
pub use crate::startup::*;
pub use crate::screen::*;
pub use crate::config::*;
pub use crate::jira::*;

fn main() {
    let config = Config::new().unwrap();
    println!("{:?}", config);
    println!("{:?}", config.companies_names());
    // start_screen();
    // let is_startup_success = startup();
    // parse_args();
}
