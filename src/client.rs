use crate::ids::ClientId;
use crate::urls::{DEV_ROOT, PROD_ROOT};
use anyhow::anyhow;
use derive_builder::Builder;
use http_types::headers::{HeaderName, HeaderValue};
use serde::Deserialize;
use std::cell::RefCell;
use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use surf::{Config, Url};

enum HeaderNames {
    Lang,
    Gateway,
    User,
    UserIp,
}

impl FromStr for HeaderNames {
    type Err = surf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X-SP-LANG" => Ok(HeaderNames::Lang),
            "X-SP-GATEWAY" => Ok(HeaderNames::Gateway),
            "X-SP-USER" => Ok(HeaderNames::User),
            "X-SP-USER-IP" => Ok(HeaderNames::UserIp),
            _ => Err(anyhow!("Invalid header {s}").into()),
        }
    }
}

impl Into<HeaderName> for HeaderNames {
    fn into(self) -> HeaderName {
        let s = match self {
            Self::Lang => "X-SP-LANG",
            Self::Gateway => "X-SP-GATEWAY",
            Self::User => "X-SP-USER",
            Self::UserIp => "X-SP-USER-IP",
        };
        HeaderName::from_str(s).unwrap()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Client {
    client_id: &'static str,
    client_secret: &'static str,
    fingerprint: &'static str,
    #[builder(default = "IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))")]
    ip_addr: IpAddr,
    #[builder(default = "false")]
    devmode: bool,
    #[builder(default = "false")]
    logger: bool,
    #[builder(default = "self.config_client()?")]
    client: RefCell<surf::Client>,
}

impl ClientBuilder {
    fn config_client(&self) -> Result<RefCell<surf::Client>, ClientBuilderError> {
        let base_url = {
            if self.devmode.unwrap() {
                DEV_ROOT
            } else {
                PROD_ROOT
            }
        };
        let client_id = self
            .client_id
            .ok_or(ClientBuilderError::UninitializedField("client_id"))?;
        let client_secret = self
            .client_secret
            .ok_or(ClientBuilderError::UninitializedField("client_secret"))?;
        let gateway = format!("{}|{}", client_id, client_secret);
        Config::new()
            .set_base_url(
                Url::parse(base_url).map_err(|_| {
                    ClientBuilderError::ValidationError("Invalid base url".to_string())
                })?,
            )
            .add_header(HeaderNames::Lang, "en")
            .map_err(|_| ClientBuilderError::ValidationError("Invalid SP-LANG header".to_string()))?
            .add_header(HeaderNames::Gateway, gateway)
            .map_err(|_| {
                ClientBuilderError::ValidationError(format!(
                    "Invalid X-SP-GATEWAY header {gateway}"
                ))
            })?
            .add_header(
                HeaderNames::UserIp,
                self.ip_addr
                    .ok_or(ClientBuilderError::UninitializedField("ip_addr"))?
                    .to_string(),
            )
            .map_err(|_| {
                ClientBuilderError::ValidationError("Invalid X-SP-USER-IP header".to_string())
            })?
            .try_into()
            .map_or(
                Err(ClientBuilderError::ValidationError(
                    "Error initializing client".to_string(),
                )),
                |c| Ok(RefCell::new(c)),
            )
    }
}

impl Client {
    fn fmt_gateway_header(&self) -> String {
        format!("{}|{}", self.client_id, self.client_secret)
    }
    fn fmt_user_header(&self, key: &str) -> String {
        format!("{}|{}", key, self.fingerprint)
    }
    pub fn update_oauth_key(&self, key: &str) {
        self.set_header(HeaderNames::User, self.fmt_user_header(key))
    }
    fn set_header(&self, name: HeaderNames, val: String) {
        let c = self.client.borrow_mut();
        let v: HeaderValue = HeaderValue::from_str(val.as_str()).unwrap();
        c.config().headers.insert(name.into(), v.into());
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientResponse {
    id: ClientId,
    name: String,
}
