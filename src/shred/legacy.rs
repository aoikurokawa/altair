use serde::{Deserialize, Serialize};

use crate::shred::ShredCommonHeader;

use super::{CodingShredHeader, DataShredHeader};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredCode {
    common_header: ShredCommonHeader,
    coding_header: CodingShredHeader,
    payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredData {
    common_header: ShredCommonHeader,
    data_header: DataShredHeader,
    payload: Vec<u8>,
}
