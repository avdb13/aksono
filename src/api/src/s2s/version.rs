use ruma::api::federation::discovery::get_server_version::v1::{Request, Response, Server};

use crate::Result;

/// `GET /_matrix/federation/*/version` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/server-server-api/#get_matrixfederationv1version
pub async fn get_server_version(_: Request) -> Result<Response> {
    Ok(Response {
        server: Some(Server {
            name: Some(env!("CARGO_PKG_NAME").to_owned()),
            version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        }),
    })
}
