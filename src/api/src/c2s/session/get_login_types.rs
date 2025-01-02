use aksono_common::Result;
use ruma::api::client::session::get_login_types::v3::{Request, Response};

/// `GET /_matrix/client/*/login` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientv3login
pub async fn get_login_types(_: Request) -> Result<Response> {
    Ok(Response::new(Vec::new()))
}
