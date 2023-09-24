use futures::future::join_all;
use reqwest;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // loop {
    //let mut handles = vec![];
    let mut pub_keys = String::new();
    for _ in 0..1023 {
        pub_keys.push_str("0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b,");
    }

    let url = format!(
        "http://127.0.0.1:8001/eth/v1/beacon/states/head/validators?id={}",
        pub_keys
    );
    // println!("{}", url);
    let res = reqwest::get(url).await.unwrap();

    println!("{:?}", res.text().await.unwrap());
}
