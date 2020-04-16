pub(crate) use bytes::buf::BufExt;
pub(crate) use hyper::client::HttpConnector;
pub(crate) use hyper::service::{make_service_fn, service_fn};
pub(crate) use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
pub(crate) use hyper_tls::HttpsConnector;
pub(crate) use chrono::Local;
pub(crate) use serde_json::json;

pub(crate) type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Result<T> = std::result::Result<T, GenericError>;