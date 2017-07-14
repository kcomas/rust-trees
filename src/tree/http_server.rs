
use hyper::header::{ContentLength, ContentType, Headers};
use hyper::server::{Request, Response, Service};
use hyper::{Method, StatusCode, Error};
use futures;

use super::container::Container;

use rand;
use rand::Rng;

pub struct HttpServer;

const HTML_NOT_FOUND: &'static [u8] = b"<h1>404</h1>";

const DEFUALT_SIZE: usize = 10;

const DEFAULT_MIN: u32 = 0;

const DEFAULT_MAX: u32 = 100;

impl Service for HttpServer {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut rsp = Response::new();
        let mut headers = Headers::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/tree") => {
                let json: String;
                if let Some(query) = req.query() {
                    json = "{}".to_string();
                } else {
                    json = generate_json_tree(DEFUALT_SIZE, DEFAULT_MIN, DEFAULT_MAX);
                }
                headers.set(ContentLength(json.len() as u64));
                headers.set(ContentType::json());
                rsp = rsp.with_headers(headers).with_body(json);
                rsp.set_status(StatusCode::Ok);
            }
            _ => {
                headers.set(ContentLength(HTML_NOT_FOUND.len() as u64));
                headers.set(ContentType::html());
                rsp = rsp.with_headers(headers).with_body(HTML_NOT_FOUND);
                rsp.set_status(StatusCode::NotFound);
            }
        };

        futures::future::ok(rsp)
    }
}

fn generate_json_tree(size: usize, min: u32, max: u32) -> String {
    let middle = (min + max) / 2;
    let cont = Container::new(middle);
    for _x in 0..size {
        let num: u32 = rand::thread_rng().gen_range(min, max);
        cont.add_value(num);
    }
    cont.to_json(true)
}
