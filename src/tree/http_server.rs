
use hyper::header::{ContentLength, ContentType, Headers};
use hyper::server::{Request, Response, Service};
use hyper::{Method, StatusCode, Error};
use futures;

pub struct HttpServer;

static HTML_NOT_FOUND: &'static [u8] = b"<h1>404</h1>";

impl Service for HttpServer {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut rsp = Response::new();
        let mut headers = Headers::new();

        match (req.method(), req.path()) {
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
