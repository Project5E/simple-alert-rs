extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn wx(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(wx))
    });
    let server = Server::bind(&addr).serve(service);
    info!("Listening on http://{}", addr);
    let graceful = server.with_graceful_shutdown((|| async {
        tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    })());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }
}