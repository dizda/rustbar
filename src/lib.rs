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

use std::process;

pub fn run(is_api_server: bool) {

    // run in cli or api
    if is_api_server == false {

        // cli
        // will simply print the result to stdout
        if let Err(e) = cli::print_to_stdout() {
            // On error, simply print out the error then exit properly
            // avoid a panic.
            println!("🤕");
            println!("---"); // avoid bitbar to switch
            println!("Application error: {}", e);
            process::exit(1);
        }
    } else {

        // launch an API server
        server::listen();
    }
}