#[macro_use] extern crate log;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

//use std::convert::Infallible;
use bytes::buf::BufExt;
//use futures_util::{stream, StreamExt};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;
use chrono::Local;
use serde_json::json;

async fn api_post_response(req: Request<Body>) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let md_req: serde_json::Value = json!({
        "msgtype": "markdown",
        "markdown": {
            "content": format!("## {}\n{}\nstate：{} [ruleUrl]({})",
                data["ruleName"].as_str().unwrap(),
                data["message"].as_str().unwrap(),
                data["state"].as_str().unwrap(),
                data["ruleUrl"].as_str().unwrap())
        }
    });

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let wx_req = Request::builder()
        .method(Method::POST)
        .uri("wx_hook")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&md_req)?.into())?;
    let wx_resp = client.request(wx_req).await?;
    let wx_resp = hyper::body::aggregate(wx_resp).await?;
    let wx_resp: serde_json::Value = serde_json::from_reader(wx_resp.reader())?;

    let json = serde_json::to_string(&wx_resp)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}


async fn response(req: Request<Body>, _client: Client<HttpConnector>, ) -> Result<Response<Body>> {
    info!("{} {} {}", Local::now().format("%T"), req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body("".into())?)
        },
        (&Method::POST, "/wx") => api_post_response(req).await,
        (&Method::POST, _) => {
            Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body("405 METHOD NOT ALLOWED".into())?)
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("404 NOT FOUND".into())?)
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();

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