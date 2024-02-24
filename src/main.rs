use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotSubscribeResponse {
    pub jsonrpc: String,
    pub method: String,
    pub params: SlotSubscribeParams,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotSubscribeParams {
    pub result: SlotSubscribeResult,
    pub subscription: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotSubscribeResult {
    pub parent: i64,
    pub root: i64,
    pub slot: i64,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // localnet
    let _rpc_url = "http://localhost:8899";
    let pub_sub = "ws://localhost:8900";

    // let mut threads = Vec::new();

    let (mut ws_stream, _res) = connect_async(pub_sub).await.expect("failed to connect");

    eprintln!("WebSocket connected");

    let req = r#"{ "jsonrpc": "2.0", "id": 1, "method": "slotSubscribe" }"#;
    ws_stream
        .send(Message::Text(req.to_owned()))
        .await
        .map_err(|_e| io::Error::new(io::ErrorKind::ConnectionRefused, ""))?;

    eprintln!("write a request");

    let _payload = ws_stream.next().await;

    while let Some(res) = ws_stream.next().await {
        match res {
            Ok(message) => match message {
                Message::Text(text) => {
                    eprintln!("{text:?}");
                    let res: SlotSubscribeResponse =
                        serde_json::from_str(&text).expect("read slot");

                    eprintln!("{res:?}");
                }
                Message::Close(close_frame) => {
                    eprintln!("Connection closed by server: {:?}", close_frame);
                    break;
                }
                _ => eprintln!("Received a non-text message"),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
