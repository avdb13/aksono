use std::sync::Arc;

use ruma::{server_util::authorization::XMatrix, ServerName};

use crate::config;

#[derive(Debug, Clone)]
pub struct Server {
    pub signing_keys: Arc<Ed25519KeyPair>,
    pub config: config::Server,
}

impl Server {
    pub fn new(signing_keys: Ed25519KeyPair, config: config::Server) -> Self {
        Self {
            signing_keys: Arc::new(signing_keys),
            config,
        }
    }

    pub async fn send_request<T>(destination: &ServerName) {
        // XMatrix::new(origin, destination.to_owned(), key, sig)

        todo!()
    }
}
