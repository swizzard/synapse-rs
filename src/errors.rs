use surf::StatusCode;
use thiserror::Error;
use crate::ids::IdType;



#[derive(Debug, Error)]
pub enum SynapseRsError {
    #[error("{0:?}")]
    SynapseError(SynapseError),
    #[error("Invalid {id_type:?} ID: {val}")]
    InvalidIdError {
        id_type: IdType,
        val: String,
    },
    #[error("Invalid zip code {0}")]
    ZipCodeError(String),
}

#[derive(Debug, Error)]
pub enum SynapseError {
    #[error("Action Pending: {message}")]
    ActionPending {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Incorrect Client Credentials: {message}")]
    IncorrectClientCredentials {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Incorrect User Credentials: {message}")]
    IncorrectUserCredentials {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Unauthorized Fingerprint: {message}")]
    UnauthorizedFingerprint {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Payload Error: {message}")]
    PayloadError {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Unauthorized Action: {message}")]
    UnauthorizedAction {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Incorrect Values: {message}")]
    IncorrectValues {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Object Not Found: {message}")]
    ObjectNotFound {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Action Not Allowed: {message}")]
    ActionNotAllowed {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Too Many Requests: {message}")]
    TooManyRequests {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Idempotency Conflict: {message}")]
    IdempotencyConflict {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Request Failed: {message}")]
    RequestFailed {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Server Error: {message}")]
    ServerError {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Service Unavailable: {message}")]
    ServiceUnavailable {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
    #[error("Gateway Timeout: {message}")]
    GatewayTimeout {
        message: String,
        http_code: StatusCode,
        error_code: u32,
        response: String,
    },
}

impl SynapseError {
    fn new(
