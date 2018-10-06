extern crate clap;
extern crate rustbar;

use clap::{Arg, App};

fn main() {

    let matches = App::new("RustBar")
        .version("0.1.0")
        .author("Jonathan Dizdarevic <dizda@dizda.fr>")
        .about("BitBar and HTTP API")
        .args(&[
            Arg::with_name("server")
                .short("s")
                .long("server")
                .help("Launch an API daemon."),
//            Arg::with_name("touch-bar")
//                .short("t")
//                .long("touch-bar")
//                .value_name("price")
//                .required(false)
//                .help("Send a price in BTC or NANO to the MacBook touch bar")
        ])
        .get_matches()
    ;

    let is_api_server = matches.is_present("server");

    rustbar::run(is_api_server);
}