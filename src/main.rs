extern crate clap;
extern crate rustbar;

use clap::{Arg, App};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = "BitBar and HTTP API.";

fn main() {

    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
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