use std::sync::Arc;

use aksono_common::app::App;
use aksono_models::entities::{access_tokens::AccessToken, devices::Device};
use axum::extract::State;
use ruma::{
    api::client::session::logout_all::v3::{Request, Response}, OwnedDeviceId,
};

use crate::{router::Sender, Result};

/// `POST /_matrix/client/*/logout/all` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3logoutall
pub async fn logout_all(
    State(app): State<Arc<App>>,
    sender: Sender<Request>,
    _: Request,
) -> Result<Response> {
    let mut conn = app.db.get().await?;

    if let Some(devices) = Device::find_by_user(&mut conn, &sender.user_id).await? {
        for device in &devices {
            let device_id: OwnedDeviceId = device.id.clone().into();

            if let Some(access_tokens) = AccessToken::find_by_device(&mut conn, &device_id).await? {
                for access_token in &access_tokens {
                    AccessToken::delete(&mut conn, &access_token.id).await?;
                }
            }

            Device::delete(&mut conn, &sender.device_id).await?;
        }
    }

    Ok(Response::new())
}
