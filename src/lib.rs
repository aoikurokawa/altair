use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use shred::{ShredDef, ShredResult};
use tokio::{
    io,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod shred;

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

    pub fn body(&self, slot: Option<usize>, indices: Option<Vec<usize>>) -> String {
        let req = match self.method {
            Method::GetShreds => {
                serde_json::json!({
                    "jsonrpc": self.jsonrpc,
                    "id": self.id,
                    "method": self.method.to_string(),
                })
            }
            Method::SlotSubscribe => {
                serde_json::json!({
                        "jsonrpc": self.jsonrpc,
                        "id": self.id,
                        "method": self.method.to_string(),
                "params":[
                    slot.unwrap(),
                    indices.unwrap(),
                    { "commitment": "confirmed" }
                ]
                    })
            }
        };

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
    pub slot: usize,
}

impl SlotSubscribe {}

pub struct Client {
    rpc: String,
}

impl Client {
    pub fn new(rpc: String) -> Self {
        Self { rpc }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let (slot_tx, slot_rx) = mpsc::unbounded_channel();
        let (shred_tx, shred_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move { Self::subscribe_slot(self, slot_tx).await });
        Ok(())
    }

    pub async fn subscribe_slot(&self, slot_tx: UnboundedSender<SlotSubscribe>) -> io::Result<()> {
        let (mut ws_stream, _res) = connect_async(&self.rpc).await.expect("failed to connect");

        let req = JsonRpcBuilder::new(Method::SlotSubscribe);
        ws_stream
            .send(Message::Text(req.body(None, None)))
            .await
            .map_err(|_e| io::Error::new(io::ErrorKind::ConnectionRefused, ""))?;

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

    pub async fn shred_update_loop(
        &self,
        slot_rx: UnboundedReceiver<SlotSubscribe>,
        shred_tx: UnboundedSender<ShredResult>,
        sample_qty: usize,
    ) -> anyhow::Result<()> {
        if let Some(slot_sub) = slot_rx.recv().await {
            let shreds = ShredDef::request_shreds(slot_sub.slot, vec![0], &self.rpc).await?;
        }

        Ok(())
    }

    pub async fn get_shreds_and_leader_for_slot() {}
}
