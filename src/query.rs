use crate::ids::{SubnetId, UserId};
use crate::node::NodeType;
use crate::scopes::Scope;
use crate::zip_code::ZipCode;
use serde::Serialize;

#[derive(Eq, PartialEq, Serialize)]
enum QueryBool {
    YES,
    NO,
}

impl From<bool> for QueryBool {
    fn from(v: bool) -> Self {
        if v {
            Self::YES
        } else {
            Self::NO
        }
    }
}

// TODO maybe: "filter"?
// see https://github.com/SynapseFI/SynapsePy/blob/966e15adc55051acaa76ff83ca6607b3c52d8efe/synapsepy/http_client.py#L80
// seems only to be used in the /billers endpoint, which SynapsePy doesn't even implement
#[derive(Serialize)]
#[serde(untagged)]
pub enum GetQuery<'a> {
    Query { query: &'a str },
    Page { page: usize },
    PerPage { per_page: usize },
    Type { r#type: NodeType },
    IsCredit { is_credit: QueryBool },
    IssuePublicKey { issue_public_key: QueryBool },
    ShowRefreshTokens { show_refresh_tokens: QueryBool },
    SubnetId { subnet_id: SubnetId },
    ForeignTransaction { foreign_transaction: QueryBool },
    FullDehydrate { full_dehydrate: QueryBool },
    ForceRefresh { force_refresh: QueryBool },
    Limit { limit: usize },
    Ticker { ticker: &'a str },
    Currency { currency: &'a str },
    Radius { radius: usize },
    Scope { scope: &'a [Scope] },
    Lat { lat: f32 },
    Lon { lon: f32 },
    Zip { zip: ZipCode },
    UserId { user_id: UserId },
}

// TODO maybe: "ship"
// https://github.com/SynapseFI/SynapsePy/blob/966e15adc55051acaa76ff83ca6607b3c52d8efe/synapsepy/http_client.py#L119
// doesn't seem used anywhere
// TODO maybe: "reset"
// https://github.com/SynapseFI/SynapsePy/blob/966e15adc55051acaa76ff83ca6607b3c52d8efe/synapsepy/http_client.py#L120
// doesn't seem used anywhere
#[derive(Serialize)]
#[serde(untagged)]
pub enum PatchQuery {
    ResendMicro { resend_micro: QueryBool },
}
