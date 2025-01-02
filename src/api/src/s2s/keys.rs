use std::{
    collections::BTreeMap,
    iter,
    sync::Arc,
    time::{Duration, SystemTime},
};

use aksono_common::app::App;
use axum::extract::State;
use ruma::{
    api::federation::discovery::{
        get_server_keys::v2::{Request, Response},
        ServerSigningKeys, VerifyKey,
    },
    serde::{Base64, Raw},
    MilliSecondsSinceUnixEpoch, OwnedServerSigningKeyId, Signatures,
};

use crate::Result;

/// `GET /_matrix/key/*/server` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/server-server-api/#get_matrixkeyv2server
pub async fn get_server_keys(State(app): State<Arc<App>>, _: Request) -> Result<Response> {
    // TODO

    let verify_keys = iter::once((
        "ed25519:123456".parse().unwrap(),
        VerifyKey::new(Base64::new(app.signing_keys.public_key().to_vec())),
    ))
    .collect();

    let valid_until_ts = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60 * 24 * 7))
        .and_then(MilliSecondsSinceUnixEpoch::from_system_time)
        .unwrap();

    Ok(Response::new(
        Raw::new(&ServerSigningKeys {
            server_name: app.config.server.name.clone(),
            verify_keys,
            old_verify_keys: BTreeMap::new(),
            signatures: Signatures::new(),
            valid_until_ts,
        })
        .unwrap(),
    ))
}
