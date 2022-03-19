use crate::client:{:Client, ClientResponse};
use crate::ids::UserId;
use crate::urls::Endpoint;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct SynapseUserResponse {
    _id: UserId,
    account_closure_date: Option<DateTime<Utc>>,
    client: ClientResponse,
    documents: Vec<Document>,
    extra: ClientExtra,
    flag: SynapseFlag,
    flag_code: Option<FlagCode>,
    permission: UserPermission,
    permission_code: Option<PermissionCode>,
    ips: Vec<IpAddr>,
    legal_names: Vec<String>,
    logins: Vec<Login>,
    phone_numbers: Vec<String>,
    refresh_token: String,
    watchlists: Watchlist,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum SynapseFlag {
    #[serde(rename = "FLAGGED")]
    Flagged,
    #[serde(rename = "NOT-FLAGGED")]
    NotFlagged,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum FlagCode {
    #[serde(rename = "ACCOUNT_CLOSURE|BLOCKED_INDUSTRY")]
    AccountClosureBlockedIndustry,
    #[serde(rename = "ACCOUNT_CLOSURE|HIGH_RISK")]
    AccountClosureHighRisk,
    #[serde(rename = "PENDING_UPLOAD|DOC_REQUEST|CIP")]
    PendingUploadDocRequestCIP,
    #[serde(rename = "PENDING_UPLOAD|DOC_REQUEST|UAR")]
    PendingUploadDocRequestUAR,
    #[serde(rename = "PENDING_UPLOAD|DOC_REQUEST|SECURITY")]
    PendingUploadDocRequestSecurity,
    #[serde(rename = "PENDING_REVIEW|DOC_REQUEST|CIP")]
    PendingReviewDocRequestCIP,
    #[serde(rename = "PENDING_REVIEW|DOC_REQUEST|UAR")]
    PendingReviewDocRequestUAR,
    #[serde(rename = "PENDING_REVIEW|DOC_REQUEST|SECURITY")]
    PendingReviewDocRequestSecurity,
    #[serde(rename = "PENDING_REVIEW|ACCOUNT_CLOSURE|BLOCKED_INDUSTRY")]
    PendingReviewAccountClosureBlockedIndustry,
    #[serde(rename = "PENDING_REVIEW|ACCOUNT_CLOSURE|HIGH_RISK")]
    PendingReviewAccountClosureHighRisk
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum UserPermission {
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "RECEIVE")]
    Receive,
    #[serde(rename = "SEND-AND-RECEIVE")]
    SendAndReceive,
    #[serde(rename = "LOCKED")]
    Locked,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "MAKE-IT-GO-AWAY")]
    MakeItGoAway,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
}
