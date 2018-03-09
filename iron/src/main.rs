
extern crate serde_json;
extern crate params;
extern crate iron;
extern crate router;
use params::FromValue;
use iron::status;
use iron::headers::ContentType;
use router::{Router};
use iron::prelude::*;
fn json(_: &mut Request) -> IronResult<Response> {
    //let json: String = serde_json::to_string(data).unwrap();
    Ok(Response::with((ContentType::html().0, status::Ok, "hi")))
}
fn main() {
    let mut router = Router::new();
    router.get("/", json, "json");
    let _server = Iron::new(router).http("0.0.0.0:80").unwrap();
    println!("lisning on localhost:8080");
}
