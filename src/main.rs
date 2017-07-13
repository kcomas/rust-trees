
extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate futures;

mod tree;
use tree::container::Container;
use std::env;

extern crate rand;
use rand::Rng;

#[macro_use]
extern crate serde_derive;

use hyper::server::Http;
use tree::http_server::HttpServer;

fn container_test() {
    let cont = Container::new(50);
    for _x in 0..10 {
        let num: u32 = rand::thread_rng().gen_range(0, 100);
        cont.add_value(num);
    }
    cont.print_tree();

    println!("{}", cont.to_json());
}

fn main() {
    container_test();

    let mut host = "0.0.0.0".to_string();
    let mut port = "3000".to_string();

    if let Ok(h) = env::var("HOST") {
        host = h;
    }

    if let Ok(p) = env::var("PORT") {
        port = p;
    }

    let str_address = format!("{}:{}", host, port);
    let addr = str_address.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HttpServer)).unwrap();
    println!("Server Started On {}", server.local_addr().unwrap());
    server.run().unwrap();
}
