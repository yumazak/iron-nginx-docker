extern crate mysql;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate iron;
extern crate router;
use iron::status;
use iron::headers::ContentType;
use router::{Router};
use iron::prelude::*;
use mysql as my;
use my::OptsBuilder;
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Jsonwrapper {
    datas: Vec<Payment>
}

fn json_handler(_: &mut Request) -> IronResult<Response> {
    fn get_opts() -> OptsBuilder {
        let mut opts = OptsBuilder::new();
        opts.ip_or_hostname(Some("db"))
            .user(Some("root"))
            // .pass(Some("mysql"))
            .db_name(Some("mydb"));
        return opts;
    }
    fn get_json() -> String{
        let opts = get_opts();
        let pool = my::Pool::new(opts).unwrap();
        let selected_payments: Vec<Payment> =
        pool.prep_exec("SELECT customer_id, amount, account_name from Payment", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                println!("{:?}", row);
                let (customer_id, amount, account_name) = my::from_row(row);
                Payment {
                    customer_id: customer_id,
                    amount: amount,
                    account_name: account_name,
                }
            }).collect()
        }).unwrap();
        let v = Jsonwrapper{datas: selected_payments};
        let p: String = serde_json::to_string(&v).unwrap();
        p
    }
    let json = get_json();
    Ok(Response::with((ContentType::json().0, status::Ok, json)))
}
fn main() {
    let mut router = Router::new();
    router.get("/json", json_handler, "json");
    let _server = Iron::new(router).http("0.0.0.0:80").unwrap();
}
