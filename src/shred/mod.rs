use anyhow::anyhow;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use solana_ledger::shred::{Error, Shred, ShredFlags, ShredType};
use solana_sdk::{signature::Signature, slot_history::Slot};

use crate::{JsonRpcBuilder, Method};

use self::{shred_code::ShredCodeDef, shred_data::ShredDataDef};

mod legacy;
mod merkle;
pub mod shred_code;
pub mod shred_data;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShredReponse {
    jsonrpc: String,
    result: ShredResult,
    id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShredResult {
    pub leader: String,
    pub shreds: Vec<Option<ShredDef>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShredDef {
    ShredCode(ShredCodeDef),
    ShredData(ShredDataDef),
}

impl TryFrom<ShredDef> for Shred {
    type Error = anyhow::Error;
    fn try_from(value: ShredDef) -> Result<Self, Self::Error> {
        match value {
            ShredDef::ShredCode(shred_code) => {
                let payload = match shred_code {
                    ShredCodeDef::Legacy(legacy) => legacy.payload,
                    ShredCodeDef::Merkle(merkle) => merkle.payload,
                };
                Shred::new_from_serialized_shred(payload)
                    .map_err(|_| anyhow!("failed to deserialize shred"))
            }
            ShredDef::ShredData(shred_data) => {
                let payload = match shred_data {
                    ShredDataDef::Legacy(legacy) => legacy.payload,
                    ShredDataDef::Merkle(merkle) => merkle.payload,
                };
                Shred::new_from_serialized_shred(payload)
                    .map_err(|_| anyhow!("failed to deserialize shred"))
            }
        }
    }
}

impl FromIterator<ShredDef> for Vec<Shred> {
    fn from_iter<T: IntoIterator<Item = ShredDef>>(iter: T) -> Self {
        let mut shreds: Vec<Shred> = Vec::new();

        for i in iter {
            let shred = Shred::try_from(i).expect("convert to Shred");
            shreds.push(shred)
        }

        shreds
    }
}

impl ShredDef {
    pub async fn request_shreds(
        slot: usize,
        indices: Vec<usize>,
        url: &str,
    ) -> anyhow::Result<ShredResult> {
        let json = JsonRpcBuilder::new(Method::GetShreds);
        let body = json.body(Some(slot as usize), Some(indices));
        let req_client = reqwest::Client::new();
        let res = req_client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?
            .text()
            .await?;

        let shred_res = serde_json::from_str::<ShredReponse>(&res)?;

        Ok(shred_res.result)

        // match shred_res.result.shreds.get(1) {
        //     Some(first_shred) => {
        //         if let Some(shred) = first_shred {
        //             // let shred: Shred = shred.to_owned().try_into()?;
        //             return Ok(shred.to_owned());
        //         } else {
        //             return Err(anyhow!("Shred not found"));
        //         }
        //     }
        //     None => {
        //         return Err(anyhow!("Shred not found"));
        //     }
        // }
    }

    pub fn num_code_shreds(&self) -> Result<u16, anyhow::Error> {
        match self {
            Self::ShredCode(code) => Ok(code.num_code_shreds()),
            Self::ShredData(_) => Err(anyhow!("invalid shred types")),
        }
    }

    pub fn num_data_shreds(&self) -> Result<u16, anyhow::Error> {
        match self {
            Self::ShredCode(code) => Ok(code.num_data_shreds()),
            Self::ShredData(_) => Err(anyhow!("invalid shred types")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct ShredCommonHeader {
    signature: Signature,
    shred_variant: ShredVariant,
    slot: Slot,
    index: u32,
    version: u16,
    fec_set_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct CodingShredHeader {
    num_data_shreds: u16,
    num_coding_shreds: u16,
    position: u16, // [0..num_coding_shreds)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(into = "u8", try_from = "u8")]
enum ShredVariant {
    LegacyCode, // 0b0101_1010
    LegacyData, // 0b1010_0101
    // proof_size is the number of Merkle proof entries, and is encoded in the
    // lowest 4 bits of the binary representation. The first 4 bits identify
    // the shred variant:
    //   0b0100_????  MerkleCode
    //   0b0110_????  MerkleCode chained
    //   0b0111_????  MerkleCode chained resigned
    //   0b1000_????  MerkleData
    //   0b1001_????  MerkleData chained
    //   0b1011_????  MerkleData chained resigned
    MerkleCode {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    }, // 0b01??_????
    MerkleData {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    }, // 0b10??_????
}

impl From<ShredVariant> for u8 {
    fn from(shred_variant: ShredVariant) -> u8 {
        match shred_variant {
            ShredVariant::LegacyCode => u8::from(ShredType::Code),
            ShredVariant::LegacyData => u8::from(ShredType::Data),
            ShredVariant::MerkleCode {
                proof_size,
                chained: false,
                resigned: false,
            } => proof_size | 0x40,
            ShredVariant::MerkleCode {
                proof_size,
                chained: true,
                resigned: false,
            } => proof_size | 0x60,
            ShredVariant::MerkleCode {
                proof_size,
                chained: true,
                resigned: true,
            } => proof_size | 0x70,
            ShredVariant::MerkleData {
                proof_size,
                chained: false,
                resigned: false,
            } => proof_size | 0x80,
            ShredVariant::MerkleData {
                proof_size,
                chained: true,
                resigned: false,
            } => proof_size | 0x90,
            ShredVariant::MerkleData {
                proof_size,
                chained: true,
                resigned: true,
            } => proof_size | 0xb0,
            ShredVariant::MerkleCode {
                proof_size: _,
                chained: false,
                resigned: true,
            }
            | ShredVariant::MerkleData {
                proof_size: _,
                chained: false,
                resigned: true,
            } => panic!("Invalid shred variant: {shred_variant:?}"),
        }
    }
}
impl TryFrom<u8> for ShredVariant {
    type Error = Error;
    fn try_from(shred_variant: u8) -> Result<Self, Self::Error> {
        if shred_variant == u8::from(ShredType::Code) {
            Ok(ShredVariant::LegacyCode)
        } else if shred_variant == u8::from(ShredType::Data) {
            Ok(ShredVariant::LegacyData)
        } else {
            let proof_size = shred_variant & 0x0F;
            match shred_variant & 0xF0 {
                0x40 => Ok(ShredVariant::MerkleCode {
                    proof_size,
                    chained: false,
                    resigned: false,
                }),
                0x60 => Ok(ShredVariant::MerkleCode {
                    proof_size,
                    chained: true,
                    resigned: false,
                }),
                0x70 => Ok(ShredVariant::MerkleCode {
                    proof_size,
                    chained: true,
                    resigned: true,
                }),
                0x80 => Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained: false,
                    resigned: false,
                }),
                0x90 => Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained: true,
                    resigned: false,
                }),
                0xb0 => Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained: true,
                    resigned: true,
                }),
                _ => Err(Error::InvalidShredVariant),
            }
        }
    }
}

/// The data shred header has parent offset and flags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct DataShredHeader {
    parent_offset: u16,
    flags: ShredFlags,
    size: u16, // common shred header + data shred header + data
}
