use std::sync::Arc;

use aksono_common::{app::App, utils};
use aksono_models::entities::{access_tokens::AccessToken, devices::Device, users::User};
use axum::extract::State;
use ruma::{
    api::client::{
        error::ErrorKind,
        session::login::v3::{LoginInfo, Request, Response, Token},
        uiaa::UserIdentifier,
    },
    DeviceId, UserId,
};

use crate::{error::Error, Result};

mod get_token;

pub use get_token::*;

/// `POST /_matrix/client/*/login` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3login
pub async fn login(State(app): State<Arc<App>>, request: Request) -> Result<Response> {
    let mut conn = app.db.get().await?;

    let Request { login_info, .. } = request;

    // TODO: check allowed login types

    let (user, user_id) = match login_info {
        LoginInfo::Password(password_info) => {
            let (user, user_id) = match &password_info.identifier {
                Some(UserIdentifier::UserIdOrLocalpart(id_or_localpart)) => {
                    let user_id = UserId::parse(id_or_localpart)
                        .or_else(|_| {
                            UserId::parse(format!(
                                "@{}:{}",
                                id_or_localpart, app.config.server_name
                            ))
                        })
                        .unwrap();

                    match User::get(&mut conn, &user_id).await {
                        Ok(user) => user.zip(Some(user_id)).ok_or_else(|| {
                            Error::BadRequest(ErrorKind::forbidden(), "User does not exist.")
                        })?,
                        Err(_) => todo!(),
                    }
                }
                _ => todo!(),
            };

            if utils::verify_password(user.password_hash.clone().unwrap(), &password_info.password)
            {
                (user, user_id)
            } else {
                todo!()
            }
        }
        LoginInfo::Token(Token { .. }) | LoginInfo::ApplicationService(_) => {
            todo!()
        }
        _ => todo!(),
    };

    let access_token = utils::secure_rand_str(32);

    // The given device ID must not be the same as a cross-signing key ID.
    let device_id = match &request.device_id {
        Some(device_id) => device_id.to_owned(),
        None => DeviceId::new(),
    };

    // If this does not correspond to a known client device, a new device will be created.
    let devices = Device::find_by_user(&mut conn, &user_id).await?;

    // Generate a new token for the device
    AccessToken::create(&mut conn, &access_token, &user_id, Some(&device_id)).await?;

    if devices
        .iter()
        .flatten()
        .all(|device| device.id != device_id)
    {
        Device::create(
            &mut conn,
            &device_id,
            &user_id,
            request.initial_device_display_name.as_deref(),
        )
        .await?;
    };

    tracing::info!(%user_id, %device_id, "User logged in");

    // Homeservers are still required to send the `home_server` field
    #[allow(deprecated)]
    Ok(Response {
        user_id,
        access_token,
        home_server: Some(app.config.server_name.clone()),
        device_id,
        well_known: None,
        refresh_token: None,
        expires_in: None,
    })
}
