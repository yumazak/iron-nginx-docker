#[macro_use]
extern crate mysql;
extern crate serde;
extern crate serde_json;
extern crate clam;
extern crate params;
#[macro_use]
extern crate serde_derive;
extern crate iron;
extern crate router;
extern crate multipart;
use params::FromValue;
use clam::template::TemplateBuilder;
use iron::status;
use iron::headers::ContentType;
use router::{Router};
use iron::prelude::*;
use mysql as my;
use my::OptsBuilder;
use std::io::Write;
#[derive(Debug, Serialize, Deserialize)]
struct BBS {
    name: Option<String>,
    body: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Jsonwrapper {
    datas: Vec<BBS>
}
fn get_opts() -> OptsBuilder {
    let mut opts = OptsBuilder::new();
    opts.ip_or_hostname(Some("db"))
        .user(Some("root"))
        .pass(Some("mysql"))
        .db_name(Some("mydb"));
    return opts;
}

fn top_handler(_: &mut Request) -> IronResult<Response> {
    let html = TemplateBuilder::new("../public/html/iron.html")
        .build()
        .html();
    html
}
fn get_items(_: &mut Request) -> IronResult<Response> {
    let opts = get_opts();
    let pool = my::Pool::new(opts).unwrap();
    let selected_payments: Vec<BBS> =
    pool.prep_exec("SELECT name, body from BBS ORDER BY id ASC", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            println!("{:?}", row);
            let (name, body) = my::from_row(row);
            BBS {
                name: name,
                body: body,
            }
        }).collect()
    }).unwrap();
    let v = Jsonwrapper{datas: selected_payments};
    let p: String = serde_json::to_string(&v).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, p)))
}

fn write(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    let opts = get_opts();
    let pool = my::Pool::new(opts).unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let name = match map.find(&["name"]) {
        Some(n) => {String::from_value(n)},
        _ => Some("".into())
    };
    let body = match map.find(&["body"]) {
        Some(d) => String::from_value(d),
        _ => Some("".into())
    };
    let bbs = BBS {name: name, body: body};
    for mut stmt in pool.prepare(r"INSERT INTO BBS
                                       (name, body)
                                   VALUES
                                       (:name, :body)").into_iter() {
        stmt.execute(params!{
            "name" => &bbs.name,
            "body" => &bbs.body,
        }).unwrap();
    }
        // let payments = vec![
        //     Payment { customer_id: 1, amount: 2, account_name: None },
        //     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        //     Payment { customer_id: 5, amount: 6, account_name: None },
        //     Payment { customer_id: 7, amount: 8, account_name: None },
        //     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
        // ];
        // for mut stmt in pool.prepare(r"INSERT INTO Payment
        //                                    (customer_id, amount, account_name)
        //                                VALUES
        //                                    (:customer_id, :amount, :account_name)").into_iter() {
        //     for p in payments.iter() {
        //         stmt.execute(params!{
        //             "customer_id" => p.customer_id,
        //             "amount" => p.amount,
        //             "account_name" => &p.account_name,
        //         }).unwrap();
        //     }
        // }
        // println!("insert succecss");
    Ok(Response::with((ContentType::html().0, status::Ok, "insert succecss")))
}
fn main() {
    let mut router = Router::new();
    router.get("/getItems", get_items, "getItems");
    router.get("/write", write, "write");
    router.get("/", top_handler, "root");
    let _server = Iron::new(router).http("0.0.0.0:80").unwrap();
    println!("lisning on localhost:8080");
}
