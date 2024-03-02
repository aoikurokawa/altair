use serde::{Deserialize, Serialize};

use super::{legacy, merkle};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShredDataDef {
    Legacy(legacy::ShredData),
    Merkle(merkle::ShredData),
}
