#[macro_use]
extern crate serde_derive;
extern crate separator;
extern crate clap;
extern crate serde_json;
extern crate hyper;

// internal files
pub mod math;
pub mod ticker;
mod util;
mod cli;
mod server;

use clap::{Arg, App};

fn main() {

    let matches = App::new("RustBar")
        .version("0.1.0")
        .author("Jonathan Dizdarevic <dizda@dizda.fr>")
        .about("BitBar and HTTP API")
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .help("Launch an API daemon.")
        )
        .get_matches()
    ;

    let is_api_server = matches.is_present("server");

    if is_api_server == false {
        cli::print_to_stdout();
    } else {
        server::listen();
    }

}