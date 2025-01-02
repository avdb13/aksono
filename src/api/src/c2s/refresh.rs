use std::{sync::Arc, time::Duration};

use aksono_common::{app::App, utils};
use aksono_models::entities::{
    access_tokens::AccessToken, refresh_tokens::RefreshToken,
};
use axum::extract::State;
use ruma::{
    api::client::{
        error::ErrorKind,
        session::refresh_token::v3::{Request, Response},
    }, OwnedDeviceId, OwnedUserId,
};

use crate::{error::Error, Result};

/// `POST /_matrix/client/*/refresh` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3refresh
pub async fn refresh(State(app): State<Arc<App>>, request: Request) -> Result<Response> {
    let mut conn = app.db.get().await?;

    let Some(refresh_token) = RefreshToken::get(&mut conn, &request.refresh_token).await? else {
        return Err(Error::BadRequest(
            ErrorKind::UnknownToken { soft_logout: true },
            "Refresh token does not exist.",
        ));
    };

    let (user_id, device_id): (OwnedUserId, OwnedDeviceId) = (
        refresh_token.user_id.parse().unwrap(),
        refresh_token.device_id.into(),
    );

    if let Some(access_tokens) = AccessToken::find_by_device(&mut conn, &device_id).await? {
        for access_token in &access_tokens {
            AccessToken::delete(&mut conn, &access_token.id).await?;
        }
    }

    let (access_token, refresh_token) = (utils::secure_rand_str(32), utils::secure_rand_str(32));

    AccessToken::create(&mut conn, &access_token, &user_id, Some(&device_id)).await?;

    let expires_in_ms = Duration::from_secs(60 * 10);

    RefreshToken::create(
        &mut conn,
        &refresh_token,
        &user_id,
        &device_id,
        utils::utc_timestamp_millis()
            .checked_add(expires_in_ms.as_millis().try_into().expect("time overflow"))
            .expect("time overflow"),
    )
    .await?;

    Ok(Response {
        access_token,
        refresh_token: Some(refresh_token),
        expires_in_ms: Some(expires_in_ms),
    })
}
