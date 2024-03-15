use std::collections::HashMap;
use std::net::SocketAddr;

use http_body_util::{BodyExt, Full};
use http_body_util::combinators::BoxBody;
use http_body_util::Empty;
use hyper::{Method, Request, Response, StatusCode};
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use serde_json::{Result as JsonResult, Value};
use tokio::net::TcpListener;
use wasm_hackathon::validate_email;

async fn handle_connection(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/hello") => Ok(Response::new(full(
            "Hello World",
        ))),
        (&Method::POST, "/api/validate_email") => {
            // Protect our server from massive bodies.
            let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
            if upper > 1024 * 64 {
                let mut resp = Response::new(full("Body too big"));
                *resp.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
                return Ok(resp);
            }

            let whole_body = req.collect().await?.to_bytes();
            if whole_body.len() == 0 {
                let mut resp = Response::new(full("Bad request"));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(resp);
            }
            let json_string = std::str::from_utf8(&whole_body).unwrap();
            let obj: HashMap<String, Value> = serde_json::from_str(json_string).unwrap();

            let email = obj.get("email").unwrap().to_string();
            println!("Received email: {:?}", email);

            let result = validate_email(&email);
            Ok(Response::new(full(Bytes::from(result.to_string()))))
        },

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(handle_connection))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

