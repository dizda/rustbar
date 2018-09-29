#[macro_use]
extern crate serde_derive;
extern crate separator;
extern crate serde_json;
extern crate hyper;

// internal files
pub mod math;
pub mod ticker;
mod util;
mod cli;
mod server;

use std::error::Error;

pub fn run(is_api_server: bool) -> Result<(), Box<dyn Error>> {
    if is_api_server == false {
        // will simply the result to stdout
        cli::print_to_stdout()?;
    } else {
        // launch an API server
        server::listen();
    }

    Ok(())
}