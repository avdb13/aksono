use std::sync::Arc;

use aksono_common::{app::App, utils, Error, Result};
use aksono_models::entities::user::User;
use axum::extract::State;
use ruma::{
    api::client::{
        session::login::v3::{LoginInfo, Password, Request, Response, Token},
        uiaa::UserIdentifier,
    },
    DeviceId, UserId,
};

/// `POST /_matrix/client/*/login` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3login
pub async fn login(State(app): State<Arc<App>>, request: Request) -> Result<Response> {
    let mut conn = app.db.get().await.unwrap();

    let Request { login_info, .. } = request;

    // TODO: check allowed login types

    let user = match login_info {
        LoginInfo::Token(Token { .. }) | LoginInfo::ApplicationService(_) => {
            unimplemented!("token/appservice login")
        }
        LoginInfo::Password(password_info) => {
            let user = match &password_info.identifier {
                Some(UserIdentifier::UserIdOrLocalpart(id_or_localpart)) => {
                    match User::find_by_localpart(&mut conn, id_or_localpart).await? {
                        Some(user) => user,
                        None => {
                            let Ok(user_id) = UserId::parse(id_or_localpart) else {
                                todo!()
                            };

                            match User::find_by_id(&mut conn, &user_id).await? {
                                Some(user) => user,
                                None => {
                                    return Err(Error::Unknown);
                                }
                            }
                        }
                    }
                }
                _ => todo!(),
            };

            match user
                .compare_password_hash(&mut conn, &password_info.password)
                .await?
            {
                Some(true) => user,
                _ => todo!(),
            }
        }
        _ => todo!(),
    };

    // If this does not correspond to a known client device, a new device will be created.
    let devices = user.get_devices(&mut conn).await?;

    // The given device ID must not be the same as a cross-signing key ID.
    let device_id = match &request.device_id {
        Some(device_id) => device_id.to_owned(),
        None => DeviceId::new(),
    };

    if devices.iter().any(|device| device.device_id == device_id) {
        todo!()
    } else {
        todo!()
    };

    // Generate a new token for the device
    let token = utils::secure_rand_str(32);

    todo!()
}
