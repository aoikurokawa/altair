use reqwest::StatusCode;
use warp::{Filter, Rejection, Reply};

#[derive(Debug)]
struct UriTooLong;

impl warp::reject::Reject for UriTooLong {}

#[derive(Debug)]
struct FooParamError;
impl warp::reject::Reject for FooParamError {}

fn uri_length_limit(limit: usize) -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    warp::any().and(warp::path::full()).and_then(
        move |route: warp::filters::path::FullPath| async move {
            if route.as_str().len() > limit {
                Err(warp::reject::custom(UriTooLong))
            } else {
                Ok(())
            }
        },
    )
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
        .map(|_, reply| reply)
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
