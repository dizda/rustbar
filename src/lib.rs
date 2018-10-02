#[macro_use] extern crate serde_derive;
extern crate separator;
extern crate serde_json;
extern crate hyper;
extern crate osascript;

// internal files
pub mod math;
pub mod ticker;
mod util;
mod cli;
mod server;

use std::process;

pub fn run(is_api_server: bool, touch_bar: &str) {

    // run in cli or api
    if is_api_server == false {

        if touch_bar != "" {
            // send to the touch bar the price
            if let Err(e) = cli::print_to_touch_bar(touch_bar) {
                // On error, simply print out the error then exit properly
                // avoid a panic.
                eprintln!("Application error: {}", e);
                process::exit(1);
            }

            // correct
            process::exit(0);
        }

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