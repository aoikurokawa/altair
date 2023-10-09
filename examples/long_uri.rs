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

fn hello_filter() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("hello" / "warp").map(|| warp::reply::html("Hello, Warp!"))
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<UriTooLong>() {
        code = StatusCode::URI_TOO_LONG;
        message = "URI Too Long";
    } else if let Some(_) = err.find::<warp::reject::UriTooLong>() {
        code = StatusCode::URI_TOO_LONG;
        message = "URI Too Long";
    } else {
        code = StatusCode::OK;
        message = "Something went wrong";
    }

    let json = warp::reply::json(&message);
    Ok(warp::reply::with_status(json, code))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let routes = uri_length_limit(10)
        .and(hello_filter())
        .map(|_, reply| reply)
        .recover(handle_rejection);

    // let hello = warp::path!("hello" / "world").map(|| warp::reply::html("Hello, World"));

    // let goodbye = warp::path!("goodbye" / "world").map(|| warp::reply::html("Goodbye, World"));

    // let route = warp::any().and(hello.or(goodbye)).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
