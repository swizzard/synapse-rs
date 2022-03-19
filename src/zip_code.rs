use crate::errors::SynapseRsError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ZipCode(String);

impl FromStr for ZipCode {
    type Err = SynapseRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Regex::new("[0-9]{5}").unwrap().is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(SynapseRsError::ZipCodeError(s.to_string()))
        }
    }
}
