use ruma::api::client::discovery::get_supported_versions::{Request, Response};

use crate::Result;

/// `GET /_matrix/client/versions` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientversions
pub async fn get_supported_versions(_: Request) -> Result<Response> {
    todo!()
}
