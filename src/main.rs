mod cli;
mod startup;
mod utils;
mod constance;
mod config;
pub use crate::cli::*;
pub use crate::startup::*;

fn main() {
    let is_startup_success = startup();
    println!("{:?}", is_startup_success);
    parse_args();
}


