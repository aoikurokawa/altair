use serde::{Deserialize, Serialize};
use solana_ledger::shred::shred_code::ShredCode;

use super::{legacy, merkle};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShredCodeDef {
    Legacy(legacy::ShredCode),
    Merkle(merkle::ShredCode),
}

// impl From<ShredCodeDef> for ShredCode {
//     fn from(value: ShredCodeDef) -> Self {
//         match value {
// 
//         }
//     }
// }
