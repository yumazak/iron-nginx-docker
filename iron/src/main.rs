extern crate clam;
extern crate iron;
extern crate router;
use clam::send;
use std::collections::HashMap;
use router::{Router};
use iron::prelude::*;
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("item", "iron");
        send::html("../public/html/iron.html", data)
    }
    let mut router = Router::new();
    router.get("/", top_handler, "root");
    let _server = Iron::new(router).http("0.0.0.0:80").unwrap();
}
