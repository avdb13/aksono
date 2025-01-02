use std::sync::Arc;

use aksono_common::{app::App, utils};
use aksono_models::entities::{
    account_data::AccountData, devices::Device, profiles::Profile,
    uiaa_credentials::UiaaCredentials, uiaa_sessions::UiaaSession, users::User,
};
use axum::extract::State;
use ruma::{
    api::client::{
        account::get_username_availability::v3::{Request, Response},
        error::ErrorKind,
        uiaa::{AuthData, AuthFlow, AuthType, Password, UiaaInfo, UserIdentifier},
    },
    CanonicalJsonObject, DeviceId, UserId,
};

use crate::{error::Error, Result};

/// `GET /_matrix/client/*/register/available` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientv3registeravailable
pub async fn get_username_availability(
    State(app): State<Arc<App>>,
    request: Request,
) -> Result<Response> {
    let mut conn = app.db.get().await?;

    let user_id = UserId::parse_with_server_name(request.username, &app.config.server.name)
        .map_err(|_error| {
            Error::BadRequest(
                ErrorKind::InvalidUsername,
                "The provided user ID is invalid.",
            )
        })?;

    let available = User::get(&mut conn, &user_id)
        .await
        .map(|user| user.is_some())?;

    Ok(Response::new(available))
}
