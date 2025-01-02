use std::sync::Arc;

use aksono_common::app::App;
use axum::extract::State;
use ruma::api::client::session::get_login_token::v1::{Request, Response};

use crate::Result;

/// `POST /_matrix/client/*/login/get_token` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv1loginget_token
pub async fn get_login_token(State(app): State<Arc<App>>, request: Request) -> Result<Response> {
    let conn = app.db.get().await?;

    todo!()

    // Ok(Response::new(expires_in, login_token))
}
