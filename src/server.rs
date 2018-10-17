use actix_web::{server, App, Json, Result, http, Path};
use serde_json;
use ticker::CoinStats;
use ticker::get_stats;
use ticker::COIN_STATS;
use cli::print_to_touch_bar;
use std::thread;
use std::time::Duration;

// interval to refresh the tickers
const REFRESH_TICKER_INTERVAL: u64 = 60;

#[derive(Deserialize)]
struct TouchbarParams {
    price: String,
}

fn index(_info: Path<()>) -> Result<String> {
    // acquire the lock on the global static in order to be thread-safe
    let coin_stats = COIN_STATS.lock().unwrap();

    Ok(serialize_response(&coin_stats))
}

/// deserialize `TouchbarParams` from request's body
fn touchbar(params: Json<TouchbarParams>) -> Result<String> {
    print_to_touch_bar(params.price.clone());

    Ok("{\"success\":true}".to_string())
}

pub fn listen() {

    // refresh ticker every 60 seconds
    spawn_refresh_ticker_thread();

    // launch API endpoint
    server::new( || App::new()
        .resource(
            "/",
            |r| r.method(http::Method::GET).with(index))
        .resource(
            "/touchbar/",
            |r| r.method(http::Method::POST).with(touchbar))
        )
        .bind("127.0.0.1:3000")
        .unwrap()
        .run()
    ;
}

// refresh the ticker fucking forever
// store the ticker result in a global static, as I don't know how to pass messages between threads.
fn spawn_refresh_ticker_thread() {
    // Run the ticker before the webserver, to be sure we'll have it in the global static.
    thread::spawn(|| {
        let make_api_call = || {
            match get_stats() {
                Ok(_r) => (), // all good
                Err(e) => println!("Thread error: {}", e)
            };

            // refresh
            thread::sleep(Duration::from_secs(REFRESH_TICKER_INTERVAL));
        };

        loop {
            // infinite calls
            make_api_call();
        }
    });
}

fn serialize_response(stats: &CoinStats) -> String {
    match serde_json::to_string(&stats) {
        Ok(json) => {
            // return a json response
            json
        }
        // This is unnecessary here because we know
        // this can't fail. But if we were serializing json that came from another
        // source we could handle an error like this.
        Err(e) => {
            eprintln!("serializing json: {}", e);

            String::new()
        }
    }
}