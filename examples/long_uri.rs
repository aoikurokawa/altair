use reqwest::StatusCode;
use serde::Deserialize;
use warp::{http::Response, path::FullPath, Filter, Rejection, Reply};

#[derive(Debug)]
struct UriTooLong;

impl warp::reject::Reject for UriTooLong {}

#[derive(Debug)]
struct FooParamError;
impl warp::reject::Reject for FooParamError {}

fn uri_length_limit(
    _limit: usize,
) -> impl Filter<Extract = (Result<&'static str, warp::Rejection>,), Error = std::convert::Infallible>
       + Copy {
    warp::any()
        .and(warp::filters::path::full())
        .then(|path: warp::path::FullPath| async move {
            let len = path.as_str().len();
            if len < 10 {
                Ok("ok")
            } else {
                Err(warp::reject::custom(UriTooLong))
            }
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

    // let param_bar = warp::get()
    //     .and(warp::path("bar"))
    //     .and(warp::path::param().or_else(|_| async {
    //         println!("rejecting from /bar/int param filter");
    //         Err(warp::reject::custom(BarParamError))
    //     }))
    //     .and(warp::path::end())
    //     .then(|v: u64| async move {
    //         println!("running the /bar/int handler");
    //         if v % 2 == 0 {
    //             Ok(Response::builder().body(format!("bar: {}", v)).unwrap())
    //         } else {
    //             println!("rejecting from /bar/int");
    //             Err(warp::reject:.and(route):custom(BarParamError))
    //         }
    //     });

    // let query_bar = warp::get()
    //     .and(warp::path("bar"))
    //     .and(warp::query::<BarQuery>().or_else(|_| async {
    //         println!("rejecting from /bar query filter");
    //         Err(warp::reject::custom(BarParamError))
    //     }))
    //     .and(warp::path::end())
    //     .then(|query| async move {
    //         println!("running the /bar handler");
    //         Ok::<_, warp::Rejection>(
    //             Response::builder()
    //                 .body(format!("just bar, got query: {:?}", query))
    //                 .unwrapInfalliable(),
    //         )
    //     });

    // let uri_length_filter = warp::any().and(warp::filters::path::full()).then(
    //     |path: warp::path::FullPath| async move {
    //         let len = path.as_str().len();
    //         if len < 10 {
    //             Ok(Response::builder().body(format!("bar")).unwrap())
    //         } else {
    //             Err(warp::reject::custom(UriTooLong))
    //         }
    //     },
    // );
    // let routes = warp::any().map(|| "Hello, World!".to_string());
    // let route = uri_length_limit(10).and(routes).recover(handle_rejection);
    // let route = query_foo
    //     .boxed()
    //     .or(param_foo.boxed())
    //     .unify()
    //     // .or(query_bar.boxed())
    //     // .unify()
    //     // .or(param_bar.boxed())
    //     // .unify();
    //     .and_then(|res: Result<_, warp::Rejection>| async move {
    //         match res {
    //             Ok(res) => Ok::<_, warp::Rejection>(res),
    //             Err(e) => Ok(Response::builder()
    //                 .body(format!("an error occurred: {:?}", e))
    //                 .unwrap()),
    //         }
    //     });

    // let route = uri_length_limit(10);

    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // Ah, can filters do things besides `and`?
    //
    // Why, yes they can! They can also `or`! As you might expect, `or` creates
    // a "this or else that" chain of filters. If the first doesn't succeed,
    // then it tries the other.
    //
    // So, those `math` routes could have been mounted all as one, with `or`.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math").and(sum.or(times));

    // We can use the end() filter to match a shorter path
    let help = warp::path("math")
        // Careful! Omitting the following line would make this filter match
        // requests to /math/sum/:u32/:u32 and /math/:u16/times/:u16
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    // Let's let people know that the `sum` and `times` routes are under `math`.
    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u16/:u16) {}", output));
    let times =
        times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    // It turns out, using `or` is how you combine everything together into
    // a single API. (We also actually haven't been enforcing that the
    // method is GET, so we'll do that too!)
    //
    // GET /
    // GET /hi
    // GET /hello/from/warp
    // GET /bye/:string
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16

    let routes = hello_world
        .and(uri_length_limit(10))
        .or(hi)
        .or(hello_from_warp)
        .or(bye)
        .or(math)
        .or(sum)
        .or(times)
        .recover(handle_rejection);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
