use std::thread;
use std::fs;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc;

use futures::channel::oneshot;
use hyper::{Body, Error, Method, Request, Response, Result, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio::runtime;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

static DST_DIR: &str = "dst";
static INDEX: &str = "index.html";
static NOT_FOUND: &[u8] = b"Not found";

pub fn serve() -> Serve {
    Serve::serve()
}

pub fn conn(addr: &SocketAddr) -> TcpStream {
    let req = TcpStream::connect(addr).unwrap();
    req.set_read_timeout(None).expect("set_read_timeout");
    req.set_write_timeout(None).expect("set_write_timeout");
    req
}

pub struct Serve {
    pub addr: SocketAddr,
    stop_tx: Option<oneshot::Sender<()>>,
    thrd: Option<thread::JoinHandle<()>>,
}

impl Serve {
    fn serve() -> Self {
        let name = format!("test-server-{}", thread::current().name().unwrap());
        let (addr_tx, addr_rx) = mpsc::channel::<SocketAddr>();
        let (stop_tx, stop_rx) = oneshot::channel::<()>();

        let thrd = thread::Builder::new()
            .name(name)
            .spawn(move || {
                let rt = runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("build");

                rt.block_on(async move {
                    let make_service = make_service_fn(|_| async {
                        Ok::<_, Error>(service_fn(response))
                    });
                    // NOTE: `:0` means any port in available
                    let addr = "127.0.0.1:0".parse().unwrap();
                    let server = Server::bind(&addr).serve(make_service);

                    addr_tx.send(server.local_addr()).expect("send");

                    let graceful = server.with_graceful_shutdown(async {
                        stop_rx.await.ok();
                        // println!("shutdown");
                    });
                    if let Err(e) = graceful.await {
                        eprintln!("err: {}", e);
                    }
                });
            })
            .expect("spawn");

        let addr = addr_rx.recv().expect("recv");
        Self {
            addr,
            stop_tx: Some(stop_tx),
            thrd: Some(thrd),
        }
    }

    pub fn shutdown(mut self) -> std::result::Result<(), ()> {
        if let Some(tx) = self.stop_tx.take() {
            tx.send(())?;
        }
        Ok(())
    }
}

impl Drop for Serve {
    fn drop(&mut self) {
        drop(self.stop_tx.take());
        drop(self.thrd.take());
    }
}

// TODO: remove
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

// TODO: remove
async fn send_file(name: &str) -> Result<Response<Body>> {
    let package_root = fs::canonicalize(".").unwrap();
    let path = package_root.as_path().join("test").join(DST_DIR).join(name);

    if let Ok(file) = File::open(path).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }
    not_found()
}

// TODO: remove
fn not_found() -> Result<Response<Body>> {
    let res = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap();
    Ok(res)
}
