#[macro_use] extern crate log;
use hyper::{Client, Server};
use hyper::service::{make_service_fn, service_fn};

mod service;
mod prelude;
use prelude::*;
use service::server::response;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:3000".parse().unwrap();

    // Share a `Client` with all `Service`s
    let client = Client::new();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    info!("Listening on http://{}", addr);
    let graceful = server.with_graceful_shutdown((|| async {
        tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    })());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }
}