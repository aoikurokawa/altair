use std::str::FromStr;

use account_proof_geyser::{types::Update, util::verify_leaves_against_bankhash};
use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};
use futures_util::{SinkExt, StreamExt};
use onchain_program::{instructions::account_hasher, CopyAccount};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use shred::{ShredDef, ShredResult};
use solana_ledger::shred::Shred;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_program, sysvar,
    transaction::Transaction,
};
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpStream,
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
    signer: Keypair,
}

#[derive(BorshSerialize)]
pub struct InstructionData {
    varint: u8,
    bump: u8,
}

impl Client {
    pub fn new(rpc: String, signer: Keypair) -> Self {
        Self { rpc, signer }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let (slot_tx, _slot_rx) = mpsc::unbounded_channel();
        let (_shred_tx, _shred_rx) = mpsc::unbounded_channel::<u8>();

        let rpc = self.rpc.clone();
        tokio::spawn(async move { Self::subscribe_slot(&rpc, slot_tx).await });
        Ok(())
    }

    pub fn query_account(&self, addr: &Pubkey) -> Account {
        let client = RpcClient::new(&self.rpc);
        client.get_account(addr).unwrap()
    }

    pub fn send_transaction(
        &self,
        program_id: &str,
        source_account: &Pubkey,
    ) -> anyhow::Result<Signature> {
        let rpc = self.rpc.clone();
        let rpc_client = RpcClient::new(rpc);
        let creator_account = self.signer.pubkey();
        let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
        let (pda, bump) = Pubkey::find_program_address(
            &[CopyAccount::SEED_PREFIX.as_bytes(), source_account.as_ref()],
            &program_id,
        );

        let instruction_data = match rpc_client.get_account(&pda) {
            Ok(_acc) => {
                println!("updating account");
                InstructionData { varint: 1, bump }
            }
            Err(_e) => {
                println!("creating account");
                InstructionData { varint: 0, bump }
            }
        };

        let accounts = vec![
            AccountMeta::new(creator_account, true),
            AccountMeta::new(*source_account, false),
            AccountMeta::new(pda, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(sysvar::clock::id(), false),
        ];

        let instruction = Instruction::new_with_borsh(program_id, &instruction_data, accounts);
        let message = solana_sdk::message::Message::new(&[instruction], Some(&creator_account));
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("get latest block hash");

        let transaction = Transaction::new(&[&self.signer], message, recent_blockhash);
        let transaction_sig = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("send and confirm transaction");

        println!("Transaction Signature: {}", transaction_sig);
        Ok(transaction_sig)
    }

    pub async fn subscribe_slot(
        rpc: &str,
        slot_tx: UnboundedSender<SlotSubscribe>,
    ) -> io::Result<()> {
        let (mut ws_stream, _res) = connect_async(rpc).await.expect("failed to connect");

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
        slot_rx: &mut UnboundedReceiver<SlotSubscribe>,
        shred_tx: UnboundedSender<ShredResult>,
        sample_qty: usize,
    ) -> anyhow::Result<()> {
        loop {
            if let Some(slot_sub) = slot_rx.recv().await {
                let res = ShredDef::request_shreds(slot_sub.slot, vec![0], &self.rpc).await?;
                let first_shred = match res.shreds.get(1) {
                    Some(first_shred) => {
                        if let Some(shred) = first_shred {
                            // let shred: Shred = shred.to_owned().try_into()?;
                            shred.to_owned()
                        } else {
                            return Err(anyhow!("Shred not found"));
                        }
                    }
                    None => {
                        return Err(anyhow!("Shred not found"));
                    }
                };

                let max_shreds_per_slot = match first_shred {
                    ShredDef::ShredCode(shred_code) => shred_code.num_code_shreds(),
                    // ShredDef::ShredData(_) => ShredDef::ShredCode(_).num_data_shreds()?,
                    ShredDef::ShredData(_) => 0,
                };

                let mut indices = gen_random_indices(max_shreds_per_slot, sample_qty);
                indices.push(0);

                let shreds_for_slot =
                    ShredDef::request_shreds(slot_sub.slot, indices, &self.rpc).await?;
                // let mut shreds = shreds_for_slot.shreds;
                // let leader = Pubkey::from_str(&shreds_for_slot.leader)?;

                // shreds.dedup();
                // shreds.iter().for_each(|shred| {
                //     if let Some(shred) = shred {
                //         if shreds
                //     }
                // })
                shred_tx.send(shreds_for_slot)?;
            }
        }
    }

    pub async fn shred_verify_loop(
        shred_rx: &mut UnboundedReceiver<ShredResult>,
        verified_shred_tx: UnboundedSender<(Shred, Pubkey)>,
    ) -> anyhow::Result<()> {
        loop {
            if let Some(shred_result) = shred_rx.recv().await {
                let shreds: Vec<Shred> = shred_result
                    .shreds
                    .into_iter()
                    .filter(|shred| shred.is_some())
                    .map(|shred| shred.unwrap())
                    .filter(|shred| Shred::try_from(shred.to_owned()).is_ok())
                    .collect();
                let leader = Pubkey::from_str(&shred_result.leader)?;

                for shred in shreds {
                    let verified = shred.verify(&leader);

                    if verified {}
                    verified_shred_tx.send((shred, leader))?;
                }
            }
        }
    }

    pub async fn get_shreds_and_leader_for_slot() {}
}

pub fn gen_random_indices(max_shreds_per_slot: u16, sample_qty: usize) -> Vec<usize> {
    let mut rng = StdRng::from_entropy();
    let indices: Vec<usize> = (0..sample_qty)
        .map(|_| rng.gen_range(0..max_shreds_per_slot as usize))
        .collect();
    indices
}

pub async fn monitor_and_verify_updates(
    rpc_pubkey: &Pubkey,
    rpc_account: &Account,
) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:10000")
        .await
        .expect("unable to connect to 127.0.0.1 on port 10000");

    let mut buffer = vec![0u8; 65536];
    let n = stream
        .read_exact(&mut buffer)
        .await
        .expect("unable to read to mutable buffer");

    if n == 0 {
        anyhow::bail!("Connection closed");
    }

    let received_update: Update = Update::deserialize(&mut &buffer[..n]).unwrap();

    let bankhash = received_update.root;
    let bankhash_proof = received_update.proof;
    let slot_num = received_update.slot;
    for p in bankhash_proof.proofs {
        verify_leaves_against_bankhash(
            &p,
            bankhash,
            bankhash_proof.num_sigs,
            bankhash_proof.account_delta_root,
            bankhash_proof.parent_bankhash,
            bankhash_proof.blockhash,
        )
        .unwrap();

        println!(
            "\nBankHash proof verification succeeded for account with Pubkey: {:?} in slot {}",
            &p.0, slot_num
        );
        let copy_account = CopyAccount::deserialize(&mut p.1 .0.account.data.as_slice())?;
        let rpc_account_hash = account_hasher(
            &rpc_pubkey,
            rpc_account.lamports,
            &rpc_account.data,
            &rpc_account.owner,
            rpc_account.rent_epoch,
        );
        assert_eq!(rpc_account_hash.as_ref(), &copy_account.digest);
        println!(
            "Hash for rpc account matches Hash verified as part of the BankHash: {}",
            rpc_account_hash
        );
        println!("{:?}", &rpc_account);
    }
    Ok(())
}
