use futures::future::join_all;
use reqwest::{self, Client, Url};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let base_url = "http://127.0.0.1:8001/eth/v1/beacon/states/head/validators";
    let client = Client::new();
    let range: Vec<u16> = (0..699).collect();
    let last = range.last().unwrap();

    let mut pub_keys = String::new();
    for val in range.clone() {
        if val == *last {
            pub_keys.push_str("0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b");
        } else {
            pub_keys.push_str("0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b,");
        }
    }

    // let params = [("id", pub_keys)];

    // let res = client
    //     .get(Url::parse(base_url).unwrap())
    //     .query(&params)
    //     .send()
    //     .await
    //     .unwrap();
    let url = format!("{}?id={}", base_url, pub_keys);
    println!("url: {}", url);

    let res = reqwest::get(url).await.unwrap();

    println!("{:?}", res.text().await.unwrap());
}
