use actix_web::{server, App, Json, Result, http, Path};
use serde_json;
use ticker::CoinStats;
use ticker::get_stats;
use cli::print_to_touch_bar;
use eventual::*;
use std::thread;
use std::time::Duration;
use redis_db;

#[derive(Deserialize)]
struct TouchbarParams {
    price: String,
}

#[derive(Serialize)]
struct JsonError {
    error: String
}

fn index(info: Path<()>) -> Result<String> {
    let con = redis_db::connection();
    let coin_stats: CoinStats = redis_db::get(&con, "ticker");

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
fn spawn_refresh_ticker_thread() {
    // Run the ticker before the webserver, to be sure we'll have it in Redis.
    thread::spawn(|| {
        let make_api_call = || {
            match get_stats() {
                Ok(_r) => (), // all good
                Err(e) => println!("Thread error: {}", e)
            };

            // refresh
            thread::sleep(Duration::from_secs(60));
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