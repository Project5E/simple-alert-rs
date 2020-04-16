use hyper::{Body, Client, Method, Request, Response, StatusCode};
use hyper::client::HttpConnector;
use chrono::Local;

use crate::prelude::*;
use crate::service::client;

pub(crate) async fn response(req: Request<Body>, _client: Client<HttpConnector>) -> Result<Response<Body>> {
    info!("{} {} {}", Local::now().format("%T"), req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body("".into())?)
        }
        (&Method::POST, "/wx") => client::api_post_response(req).await,
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
