use reqwest::StatusCode;
use serde::Deserialize;
use warp::{http::Response, path::FullPath, Filter, Rejection, Reply};

#[derive(Debug)]
struct UriTooLong;

impl warp::reject::Reject for UriTooLong {}

#[derive(Debug)]
struct FooParamError;
impl warp::reject::Reject for FooParamError {}

// fn uri_length_limit(
//     limit: usize,
// ) -> impl Filter<
//     Extract = (Result<Response<String>, warp::Rejection>,),
//     Error = std::convert::Infallible,
// > + Copy {
// }

// async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
//     let code;
//     let message;
//
//     if let Some(_) = err.find::<UriTooLong>() {
//         code = StatusCode::URI_TOO_LONG;
//         message = "URI Too Long";
//     } else {
//         code = StatusCode::OK;
//         message = "";
//     }
//
//     let json = warp::reply::json(&message);
//     Ok(warp::reply::with_status(json, code))
// }

fn query_filter(
) -> impl warp::Filter<Extract = (Result<String, warp::Rejection>,), Error = warp::Rejection> + Copy
{
    warp::filters::query::raw().then(|query_str: String| async move {
        if query_str.len() > 5 {
            Ok(query_str)
        } else {
            Err(warp::reject::custom(FooQueryError))
        }
    })
}

#[derive(Debug)]
struct FooQueryError;
impl warp::reject::Reject for FooQueryError {}

#[derive(Debug)]
struct BarParamError;
impl warp::reject::Reject for BarParamError {}

#[derive(Debug)]
struct BarQueryError;
impl warp::reject::Reject for BarQueryError {}

#[derive(Debug, Deserialize)]
struct BarQuery {
    id: u64,
}

#[tokio::main]
async fn main() {
    let param_foo = warp::get()
        .and(warp::path("foo"))
        .and(warp::path::param().or_else(|_| async {
            println!("rejecting from /foo/int param filter");
            Err(warp::reject::custom(FooParamError))
        }))
        .and(warp::path::end())
        .then(|v: u64| async move {
            println!("running the /foo/int handler");
            if v % 2 == 0 {
                Ok(Response::builder().body(format!("foo int: {}", v)).unwrap())
            } else {
                println!("rejecting from /foo/int");
                Err(warp::reject::custom(FooParamError))
            }
        });

    let query_foo = warp::get()
        .and(warp::path("foo"))
        .and(query_filter())
        .and(warp::path::end())
        .then(|query| async {
            println!("running the /foo handler");
            let query_str = query?;
            Ok::<_, warp::Rejection>(
                Response::builder()
                    .body(format!("just foo, got query: {}", query_str))
                    .unwrap(),
            )
        });

    let param_bar = warp::get()
        .and(warp::path("bar"))
        .and(warp::path::param().or_else(|_| async {
            println!("rejecting from /bar/int param filter");
            Err(warp::reject::custom(BarParamError))
        }))
        .and(warp::path::end())
        .then(|v: u64| async move {
            println!("running the /bar/int handler");
            if v % 2 == 0 {
                Ok(Response::builder().body(format!("bar: {}", v)).unwrap())
            } else {
                println!("rejecting from /bar/int");
                Err(warp::reject::custom(BarParamError))
            }
        });

    let query_bar = warp::get()
        .and(warp::path("bar"))
        .and(warp::query::<BarQuery>().or_else(|_| async {
            println!("rejecting from /bar query filter");
            Err(warp::reject::custom(BarParamError))
        }))
        .and(warp::path::end())
        .then(|query| async move {
            println!("running the /bar handler");
            Ok::<_, warp::Rejection>(
                Response::builder()
                    .body(format!("just bar, got query: {:?}", query))
                    .unwrapInfalliable(),
            )
        });

    let uri_length_filter = warp::any().and(warp::filters::path::full()).then(
        move |path: warp::path::FullPath| async move {
            let len = path.as_str().len();
            if len < 10 {
                Ok(Response::builder().body(format!("ok")).unwrap())
            } else {
                Err(warp::reject::custom(UriTooLong))
            }
        },
    );

    let route = query_foo
        .boxed()
        .or(param_foo.boxed())
        .unify()
        .or(query_bar.boxed())
        .unify()
        .or(param_bar.boxed())
        .unify()
        .and_then(|res: Result<_, warp::Rejection>| async move {
            match res {
                Ok(res) => Ok::<_, warp::Rejection>(res),
                Err(e) => Ok(Response::builder()
                    .body(format!("an error occurred: {:?}", e))
                    .unwrap()),
            }
        });

    let route = uri_length_filter.and(route);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
