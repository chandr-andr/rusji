mod cli;
mod startup;
mod utils;
mod constance;
mod config;
mod actions;
pub use crate::cli::*;
pub use crate::startup::*;
pub use crate::actions::*;
pub use crate::config::*;

fn main() {
    start_screen();
    let is_startup_success = startup();
    parse_args();
}


