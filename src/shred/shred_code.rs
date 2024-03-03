use serde::{Deserialize, Serialize};

use super::{legacy, merkle};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShredCodeDef {
    Legacy(legacy::ShredCode),
    Merkle(merkle::ShredCode),
}

impl ShredCodeDef {
    pub fn num_code_shreds(&self) -> u16 {
        match self {
            Self::Legacy(legacy) => legacy.coding_header.num_coding_shreds,
            Self::Merkle(merkle) => merkle.coding_header.num_coding_shreds,
        }
    }

    pub fn num_data_shreds(&self) -> u16 {
        match self {
            Self::Legacy(legacy) => legacy.coding_header.num_data_shreds,
            Self::Merkle(merkle) => merkle.coding_header.num_data_shreds,
        }
    }
}

// impl From<ShredCodeDef> for ShredCode {
//     fn from(value: ShredCodeDef) -> Self {
//         match value {
//
//         }
//     }
// }
