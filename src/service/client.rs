use bytes::buf::BufExt;
use hyper::{header, Body, Client, Method, Request, Response, StatusCode};
use hyper_tls::HttpsConnector;
use serde_json::json;

use crate::prelude::*;
use crate::service::Webhook;

pub(crate) async fn api_post_response(req: Request<Body>, webhook: Webhook) -> Result<Response<Body>> {
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
        .uri(webhook.wx)
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
