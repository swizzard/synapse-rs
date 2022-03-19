use crate::errors::SynapseRsError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum IdType {
    Client,
    Node,
    Subnet,
    User,
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UserId(String);

impl FromStr for UserId {
    type Err = SynapseRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_id(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(SynapseRsError::InvalidIdError {
                id_type: IdType::User,
                val: s.to_string(),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NodeId(String);

impl FromStr for NodeId {
    type Err = SynapseRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_id(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(SynapseRsError::InvalidIdError {
                id_type: IdType::Node,
                val: s.to_string(),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SubnetId(String);

impl FromStr for SubnetId {
    type Err = SynapseRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_id(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(SynapseRsError::InvalidIdError {
                id_type: IdType::Subnet,
                val: s.to_string(),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ClientId(String);

impl FromStr for ClientId {
    type Err = SynapseRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_id(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(SynapseRsError::InvalidIdError {
                id_type: IdType::Client,
                val: s.to_string(),
            })
        }
    }
}

fn validate_id(s: &str) -> bool {
    Regex::new("[0-9a-f]{24}").unwrap().is_match(s)
}
