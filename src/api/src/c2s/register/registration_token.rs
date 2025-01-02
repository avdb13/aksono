use std::{sync::Arc, time::Instant};

use aksono_common::{app::App, Result};
use aksono_models::entities::registration_token::RegistrationToken;
use axum::extract::State;
use ruma::api::client::account::check_registration_token_validity::v1::{Request, Response};

/// `GET /_matrix/client/*/register/m.login.registration_token/validity` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientv1registermloginregistration_tokenvalidity
pub async fn get_registration_token_validity(
    State(app): State<Arc<App>>,
    request: Request,
) -> Result<Response> {
    let mut conn = app.db.get().await.unwrap();

    let registration_token = RegistrationToken::find_by_token(&mut conn, &request.token).await?;

    Ok(Response::new(registration_token.map_or(false, |rt| {
        // TODO: timezones
        rt.expiry_time.map_or(false, |ts| {
            u128::from(ts.unsigned_abs()) < Instant::now().elapsed().as_millis()
        })
    })))
}
