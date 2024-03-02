use serde::{Deserialize, Serialize};

use super::{CodingShredHeader, DataShredHeader, ShredCommonHeader};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredData {
    common_header: ShredCommonHeader,
    data_header: DataShredHeader,
    payload: Vec<u8>,
}

// Layout: {common, coding} headers | erasure coded shard
//     | [Merkle root of the previous erasure batch if chained]
//     | Merkle proof
//     | [Retransmitter's signature if resigned]
// The slice past signature and before the merkle proof is hashed to generate
// the Merkle tree. The root of the Merkle tree is signed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShredCode {
    common_header: ShredCommonHeader,
    coding_header: CodingShredHeader,
    payload: Vec<u8>,
}
