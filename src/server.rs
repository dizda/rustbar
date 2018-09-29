use hyper::{Body, Response, Server, header, StatusCode};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use serde_json;
use ticker;
use ticker::CoinStats;

pub fn listen() {
//    pretty_env_logger::init();

    let addr = ([0, 0, 0, 0], 3000).into();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = || {
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function that
        // returns a Response into a `Service`.
        service_fn_ok(|_| {

            // generate stats
            let stats = match ticker::get_stats() {
                Ok(s)  => s,
                Err(e) => {
                    eprintln!("Can't call APIs: {}", e);

                    // cast error from std::error::Error to String
                    let error = JsonError { error: e.to_string() };
                    // serialize the error to JSON
                    let json_err = serde_json::to_string(&error).unwrap();

                    // return a proper 500 page, with the error response if any
                    return Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(json_err))
                                    .unwrap();
                }

            };

            // serialize body and create the response
            serialize_response(&stats)

        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

#[derive(Serialize)]
struct JsonError {
    error: String
}

fn serialize_response(stats: &CoinStats) -> Response<Body> {
    match serde_json::to_string(&stats) {
        Ok(json) => {
            // return a json response
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap()
        }
        // This is unnecessary here because we know
        // this can't fail. But if we were serializing json that came from another
        // source we could handle an error like this.
        Err(e) => {
            eprintln!("serializing json: {}", e);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}