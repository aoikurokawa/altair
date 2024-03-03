use serde::{Deserialize, Serialize};

use crate::shred::ShredCommonHeader;

use super::{CodingShredHeader, DataShredHeader};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredCode {
    pub common_header: ShredCommonHeader,
    pub coding_header: CodingShredHeader,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredData {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}
