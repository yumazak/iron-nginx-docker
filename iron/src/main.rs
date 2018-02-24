extern crate clam;
extern crate iron;
extern crate router;
use clam::template;
use std::collections::HashMap;
use router::{Router};
use iron::prelude::*;
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("item", "iron");
        template::add(data,"../public/html/iron")
    }
    let mut router = Router::new();
    router.get("/", top_handler, "root");
    let _server = Iron::new(router).http("0.0.0.0:80").unwrap();
}
