extern crate clap;
extern crate rustbar;

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

    rustbar::run(is_api_server);
}