use std::sync::Arc;

use aksono_common::app::App;
use axum::extract::State;
use ruma::{
    api::client::account::register::{
        v3::{Request, Response},
        RegistrationKind,
    },
    DeviceId,
};

use crate::Result;

/// `POST /_matrix/client/*/register` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3register
pub async fn register(State(_app): State<Arc<App>>, request: Request) -> Result<Response> {
    match request.kind {
        RegistrationKind::Guest => todo!(),
        RegistrationKind::User => {}
        _ => todo!(),
    };

    // TODO: given device ID must not be the same as a cross-signing key ID.
    let _device_id = match &request.device_id {
        Some(device_id) => device_id.to_owned(),
        None => DeviceId::new(),
    };

    todo!()
}
