use actix_web::{server, App, Json, Result, http, Path};
use serde_json;
use ticker;
use ticker::CoinStats;
use cli::print_to_touch_bar;

#[derive(Deserialize)]
struct TouchbarParams {
    price: String,
}

#[derive(Serialize)]
struct JsonError {
    error: String
}

fn index(info: Path<()>) -> Result<String> {
    let coin_stats = match ticker::get_stats() {
        Ok(s)  => s,
        Err(e) => {
            eprintln!("Can't call APIs: {}", e);

            // cast error from std::error::Error to String
            let error = JsonError { error: e.to_string() };
            // serialize the error to JSON
            let json_err = serde_json::to_string(&error).unwrap();

            return Ok(json_err);
        }
    };

    Ok(serialize_response(&coin_stats))
}

/// deserialize `Info` from request's body
fn touchbar(params: Json<TouchbarParams>) -> Result<String> {
    print_to_touch_bar(params.price.clone());

    Ok("{\"success\":true}".to_string())
}

pub fn listen() {

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