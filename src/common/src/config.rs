use std::{fmt::Display, net::SocketAddr, ops::Deref, path::PathBuf};

use ruma::{api::federation::discovery::OldVerifyKey, OwnedServerName, OwnedServerSigningKeyId};
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub server: Server,
    pub listener: Listener,
    pub signing_keys: SigningKeys,
    pub discovery: Discovery,
    pub registration: Registration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub name: OwnedServerName,
    pub public_url: url::Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SigningKeys {
    pub path: PathBuf,
    pub old: Vec<(OwnedServerSigningKeyId, OldVerifyKey)>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Discovery {
    pub base_url: Option<url::Url>,
    pub support_page: Option<url::Url>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Registration {
    pub users: bool,
    #[serde(default)]
    pub guests: bool,
}

#[derive(Debug, Clone)]
pub struct Listener(SocketAddr);

impl Deref for Listener {
    type Target = SocketAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for Listener {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Dummy {
            pub addr: String,
            pub port: u16,
        }

        let dummy = Dummy::deserialize(deserializer)?;

        let (addr, port) = (dummy.addr.parse().map_err(Error::custom)?, dummy.port);

        Ok(Self(SocketAddr::new(addr, port)))
    }
}
