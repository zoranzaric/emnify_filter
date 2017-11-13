extern crate hyper;
extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use std::str;

use futures::future::Future;
use futures::Stream;

use hyper::{Client, Method, Request, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, Service};


fn send(url: hyper::Uri, data: hyper::Chunk) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure().build(&handle);

    let mut request = Request::new(Method::Post, url);
    request.headers_mut().set(ContentType::json());
    request.headers_mut().set(ContentLength(data.len() as u64));
    request.set_body(data);

    let work = client.request(request);
    core.run(work).unwrap();
}


struct RecieveEvent;

#[derive(Deserialize)]
struct EventType {
    id: i32,
}

#[derive(Deserialize)]
struct RequestData {
    event_type: EventType,
}

impl Service for RecieveEvent {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/") => {
                Box::new(req.body().concat2().and_then(|data| {
                    let v: RequestData = serde_json::from_slice(&data).unwrap();
                    if v.event_type.id == 6 {
                        println!("!");
                        send("http://127.0.0.1:5000/".parse::<hyper::Uri>().unwrap(), data)
                    } else {
                        println!(".")
                    }
                    futures::future::ok(Response::new())
                }))
            }
            _ => {
                let mut response = Response::new();
                response.set_status(StatusCode::NotFound);
                response.set_body("");
                Box::new(futures::future::ok(response))
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:6666".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(RecieveEvent)).unwrap();
    server.run().unwrap();
}
