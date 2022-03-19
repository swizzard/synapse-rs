use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename(serialize = "USERS|POST", deserialize = "USERS|POST"))]
    UsersPost,
    #[serde(rename(serialize = "USER|PATCH", deserialize = "USERS|PATCH"))]
    UsersPatch,
    #[serde(rename(serialize = "NODES|POST", deserialize = "NODES|POST"))]
    NodesPost,
    #[serde(rename(serialize = "NODE|PATCH", deserialize = "NODE|PATCH"))]
    NodePatch,
    #[serde(rename(serialize = "TRANS|POST", deserialize = "TRANS|POST"))]
    TransPost,
    #[serde(rename(serialize = "TRAN|PATCH", deserialize = "TRAN|PATCH"))]
    TranPatch,
    #[serde(rename(serialize = "SUBNETS|POST", deserialize = "SUBNETS|POST"))]
    SubnetsPost,
    #[serde(rename(serialize = "SUBNET|PATCH", deserialize = "SUBNET|PATCH"))]
    SubnetPatch,
}
