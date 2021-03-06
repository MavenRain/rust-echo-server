extern crate futures;
extern crate hyper;

use futures::{future, Future, Stream};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

type FutureResponse = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn echo(request: Request<Body>) -> FutureResponse {
  Box::new(future::ok(
    match (request.method(), request.uri().path()) {
      (& Method::GET, "/") => Response::new(Body::from("Try POSTing data to /echo")),
      (& Method::POST, "/echo") => Response::new(request.into_body()),
      (& Method::POST, "/echo/uppercase") => Response::new(Body::wrap_stream(
        request.into_body().map(|chunk| chunk.iter().map(|byte| byte.to_ascii_uppercase())
          .collect::<Vec<u8>>())
      )),
      _ => Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("Path was not found"))
        .unwrap_or(Response::new(Body::from("")))
    }
  ))
}

fn main() {
  let address = ([0, 0, 0, 0], 8080).into();
  hyper::rt::run(Server::bind(& address).serve(|| service_fn(echo))
    .map_err(|error| println!("Server error: {}", error)));
}
