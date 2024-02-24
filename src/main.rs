use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{self},
    sync::mpsc::{self, Sender, UnboundedSender},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub enum Method {
    GetShreds,
    SlotSubscribe,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::GetShreds => "getShreds".to_string(),
            Self::SlotSubscribe => "slotSubscribe".to_string(),
        }
    }
}

pub struct JsonRpcBuilder {
    jsonrpc: f32,
    id: u8,
    method: Method,
}

impl JsonRpcBuilder {
    pub fn new(method: Method) -> Self {
        Self {
            jsonrpc: 2.0,
            id: 1,
            method,
        }
    }
}

impl ToString for JsonRpcBuilder {
    fn to_string(&self) -> String {
        let req = serde_json::json!({
            "jsonrpc": self.jsonrpc,
            "id": self.id,
            "method": self.method.to_string(),
        });

        req.to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotSubscribeResponse {
    pub jsonrpc: String,
    pub method: String,
    pub params: SlotSubscribeParams,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotSubscribeParams {
    pub result: SlotSubscribe,
    pub subscription: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotSubscribe {
    pub parent: i64,
    pub root: i64,
    pub slot: i64,
}

impl SlotSubscribe {
    pub fn request_shreds(&self, indices: Vec<usize>) {
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getShreds",
            "params": [
                self.slot,
                indices,
                { "commitment": "confirmed" }
            ]
        });
    }
}

pub struct Client {
    rpc: String,
}

impl Client {
    pub fn new(rpc: String) -> Self {
        Self { rpc }
    }

    pub async fn subscribe_slot(&self, slot_tx: UnboundedSender<SlotSubscribe>) -> io::Result<()> {
        let (mut ws_stream, _res) = connect_async(&self.rpc).await.expect("failed to connect");

        // let req = r#"{ "jsonrpc": "2.0", "id": 1, "method": "slotSubscribe" }"#;
        let req = JsonRpcBuilder::new(Method::SlotSubscribe);
        ws_stream
            .send(Message::Text(req.to_string()))
            .await
            .map_err(|_e| io::Error::new(io::ErrorKind::ConnectionRefused, ""))?;

        eprintln!("write a request");

        let _payload = ws_stream.next().await;

        while let Some(res) = ws_stream.next().await {
            match res {
                Ok(message) => match message {
                    Message::Text(text) => {
                        let res: SlotSubscribeResponse =
                            serde_json::from_str(&text).expect("read slot");

                        slot_tx.send(res.params.result).expect("send slot result");
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

    pub async fn get_shreds_and_leader_for_slot() {}
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // localnet
    let _rpc_url = "http://localhost:8899";
    let client = Client::new("ws://localhost:8900".to_string());

    let (slot_tx, slot_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move { client.subscribe_slot(slot_tx).await });

    // let mut threads = Vec::new();

    Ok(())
}
