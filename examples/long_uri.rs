use std::net::SocketAddr;

use reqwest::StatusCode;
use serde::Deserialize;
use warp::{http::Response, path::FullPath, Filter, Rejection, Reply};

#[derive(Debug)]
struct UriTooLong;

impl warp::reject::Reject for UriTooLong {}

#[derive(Debug)]
struct FooParamError;
impl warp::reject::Reject for FooParamError {}

fn uri_length_limit(limit: usize) -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::filters::addr::remote())
        .and_then(move |addr: Option<SocketAddr>| {
            if let Some(a) = addr {
                let length = a.ip().to_string().len();
                if length > limit {
                    return futures::future::err(warp::reject::custom(UriTooLong));
                }
            }
            futures::future::ok(())
        })
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<UriTooLong>() {
        code = StatusCode::URI_TOO_LONG;
        message = "URI Too Long";
    } else {
        code = StatusCode::OK;
        message = "";
    }

    let json = warp::reply::json(&message);
    Ok(warp::reply::with_status(json, code))
}

fn hello_filter() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("hello" / "warp").map(|| warp::reply::html("Hello, Warp!"))
}

#[tokio::main]
async fn main() {
    let routes = uri_length_limit(10)
        .and(hello_filter())
        .recover(handle_rejection);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
