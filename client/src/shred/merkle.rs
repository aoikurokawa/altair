use serde::{Deserialize, Serialize};

use super::{CodingShredHeader, DataShredHeader, ShredCommonHeader};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredData {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}

// Layout: {common, coding} headers | erasure coded shard
//     | [Merkle root of the previous erasure batch if chained]
//     | Merkle proof
//     | [Retransmitter's signature if resigned]
// The slice past signature and before the merkle proof is hashed to generate
// the Merkle tree. The root of the Merkle tree is signed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredCode {
    pub common_header: ShredCommonHeader,
    pub coding_header: CodingShredHeader,
    pub payload: Vec<u8>,
}
