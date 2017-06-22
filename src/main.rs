#![deny(warnings)]
extern crate hyper;
extern crate futures;

use futures::future::FutureResult;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

static PHRASE: &'static [u8] = b"mirei";

struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE)
        )
    }
}

fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Hello)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
