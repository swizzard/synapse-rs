pub(crate) const DEV_ROOT: &str = "https://uat-api.synapsefi.com/v3.1";
pub(crate) const PROD_ROOT: &str = "https://api.synapsefi.com/v3.1";

const OAUTH: &str = "/oauth";
const CLIENT: &str = "/client";
const USERS: &str = "/users";
const TRANS: &str = "/trans";
const NODES: &str = "/nodes";
const SUBS: &str = "/subscriptions";
const SUBN: &str = "/subnets";
const INST: &str = "/institutions";
const APPLE: &str = "/applepay";
const ATMS: &str = "/atms";
const CRYPTOQ: &str = "/crypto-quotes";
const CRYPTOM: &str = "/crypto-market-watch";
const DUMMY: &str = "/dummy-tran";
const DISPUTE: &str = "/dispute";
const UBO: &str = "/ubo";
const LOGS: &str = "/logs";
const SHIP: &str = "/ship";
const STATEMENTS: &str = "/statements";
const TRADE: &str = "/trade-market-watch";

const LONGEST_URL_LEN: usize = 54;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Endpoint {
    Oauth,
    Client,
    Users,
    Trans,
    Nodes,
    Subs,
    Subn,
    Inst,
    Apple,
    CryptoQ,
    CryptoM,
    Dummy,
    Dispute,
    Ubo,
    Logs,
    Ship,
    Statements,
    Trade,
}

impl Endpoint {
    pub fn get_url(&self) -> &'static str {
        match self {
            Oauth => OAUTH,
            Client => CLIENT,
            Users => USERS,
            Trans => TRANS,
            Nodes => NODES,
            Subs => SUBS,
            Subn => SUBN,
            Inst => INST,
            Apple => APPLE,
            CryptoQ => CRYPTOQ,
            CryptoM => CRYPTOM,
            Dummy => DUMMY,
            Dispute => DISPUTE,
            Ubo => UBO,
            Logs => LOGS,
            Ship => SHIP,
            Statements => STATEMENTS,
            Trade => TRADE,
        }
    }
}
