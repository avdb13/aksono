use std::{iter, sync::Arc, time::Duration};

use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager, ManagerConfig},
    AsyncPgConnection,
};
use ruma::{
    api::{client::account::register::RegistrationKind, federation::discovery::VerifyKey},
    serde::Base64,
    signatures::Ed25519KeyPair,
    OwnedServerSigningKeyId,
};
use tokio::fs::File;

use crate::config;

#[derive(Debug, Clone)]
pub struct App {
    pub config: config::Config,
    pub db: Pool<AsyncPgConnection>,
    pub signing_keys: Arc<Ed25519KeyPair>,
}

impl App {
    pub async fn new(config: config::Config) -> Self {
        let db = {
            let builder = Pool::builder().max_size(8).min_idle(Some(4));

            let (max_lifetime, idle_timeout) = (
                Some(Duration::from_secs(60 * 60 * 24)),
                Some(Duration::from_secs(60 * 2)),
            );

            let builder = builder
                .max_lifetime(max_lifetime)
                .idle_timeout(idle_timeout);

            let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
                std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                ManagerConfig::default(),
            );

            builder.build(manager).await.unwrap()
        };

        // TODO
        // let signing_keys = {
        //     match tokio::fs::read(&config.signing_keys.path).await {
        //         Ok(contents) => {
        //             let mut it = contents.split(|b| b == b' ');

        //             let alg_and_version = [it.nth(0).unwrap()];
        //             let encoded_key = it.nth(2).unwrap();

        //             todo!()
        //         }
        //         Err(error) => {
        //             if error.kind() != std::io::ErrorKind::NotFound {}

        //             todo!()
        //         }
        //     }
        // };

        let keypair = Ed25519KeyPair::generate().unwrap();

        Self {
            config,
            db,
            signing_keys: Arc::new(
                Ed25519KeyPair::from_der(keypair.as_slice(), String::new()).unwrap(),
            ),
        }
    }

    pub fn registration_disabled(&self, kind: RegistrationKind) -> bool {
        use RegistrationKind::*;

        !self.config.registration.users && kind == User
            || !self.config.registration.guests && kind == Guest
    }
}
