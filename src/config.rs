use std::{net::SocketAddr, ops::Deref};

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_name: String,
    pub listener: Listener,
}

#[derive(Debug, Clone)]
pub struct Listener(SocketAddr);

impl Deref for Listener {
    type Target = SocketAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
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
