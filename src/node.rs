use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum NodeType {
    #[serde(rename(serialize = "ACH-US", deserialize = "ACH-US"))]
    ACH,
    #[serde(rename(serialize = "INTERCHANGE-US", deserialize = "INTERCHANGE-US"))]
    INTERCHANGE,
    #[serde(rename(serialize = "CLEARING-US", deserialize = "CLEARING-US"))]
    CLEARING,
    #[serde(rename(serialize = "CUSTODY-US", deserialize = "CUSTODY-US"))]
    CUSTODY,
    #[serde(rename(serialize = "DEPOSIT-US", deserialize = "DEPOSIT-US"))]
    DEPOSIT,
    #[serde(rename(serialize = "CHECK-US", deserialize = "CHECK-US"))]
    CHECK,
}
