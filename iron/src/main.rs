extern crate clam;
extern crate iron;
use clam::template::TemplateBuilder;
use std::collections::HashMap;
use iron::prelude::*;
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("item", "hoge");
        let html = TemplateBuilder::new("../public/html/iron.html")
            .data(data)
            .build()
            .html();
        html
    }
    let _server = Iron::new(top_handler).http("localhost:3000").unwrap();
}
