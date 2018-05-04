extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate env_logger;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use actix_web::{
    middleware, http, server,
    App, AsyncResponder, HttpRequest, HttpResponse, HttpMessage, Error, Json};

use futures::{Future};

#[derive(Debug, Serialize, Deserialize)]
struct ApiCall {
    cmd: String,
    param: String,
}

/// This handler uses `HttpRequest::json()` for loading json object.
fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()  // convert all errors into `Error`
        .and_then(|val: ApiCall| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}

/// This handler uses json extractor
fn extract_item(item: Json<ApiCall>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)  // <- send response
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("json-example");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/extractor", |r| {
                r.method(http::Method::POST)
                    .with(extract_item)
                    .limit(4096); // <- limit size of the payload
            })
            .resource("/", |r| r.method(http::Method::POST).f(index))})
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
