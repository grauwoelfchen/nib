use std::path::Path;

use hyper::{Body, Error, Method, Request, Response, Result, StatusCode};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

static DST_DIR: &str = "dst";
static INDEX: &str = "index.html";
static NOT_FOUND: &[u8] = b"Not found";

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let make_service =
        make_service_fn(|_| async { Ok::<_, Error>(service_fn(response)) });
    let server = Server::bind(&addr).serve(make_service);
    println!("listening 127.0.0.1:3000...");

    if let Err(e) = server.await {
        eprintln!("err: {}", e);
    }
}

/// returns a result conatins result response body if it's expected.
async fn response(req: Request<Body>) -> Result<Response<Body>> {
    let path = req.uri().path();
    match (req.method(), path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => {
            send_file(INDEX).await
        }
        (&Method::GET, path) => send_file(&path[1..]).await,
        _ => not_found(),
    }
}

/// returns a result contains file body from dist directory or just an error of
/// not found.
async fn send_file(name: &str) -> Result<Response<Body>> {
    let path = Path::new(DST_DIR).join(name);

    if let Ok(file) = File::open(path).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }
    not_found()
}

/// returns a result contains not found error.
fn not_found() -> Result<Response<Body>> {
    let res = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap();
    Ok(res)
}
