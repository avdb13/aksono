use std::sync::Arc;

use aksono_common::app::App;
use aksono_models::entities::{access_tokens::AccessToken, devices::Device};
use axum::extract::State;
use ruma::api::client::session::logout::v3::{Request, Response};

use crate::{router::Sender, Result};

mod all;

pub use all::*;

/// `POST /_matrix/client/*/logout` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3logout
pub async fn logout(
    State(app): State<Arc<App>>,
    sender: Sender<Request>,
    _: Request,
) -> Result<Response> {
    let mut conn = app.db.get().await?;

    if let Some(access_tokens) = AccessToken::find_by_device(&mut conn, &sender.device_id).await? {
        for access_token in &access_tokens {
            AccessToken::delete(&mut conn, &access_token.id).await?;
        }
    }

    Device::delete(&mut conn, &sender.device_id).await?;

    Ok(Response::new())
}
