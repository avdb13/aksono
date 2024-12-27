use std::sync::Arc;

use aksono_common::app::App;
use axum::extract::State;
use ruma::api::client::discovery::discover_homeserver::{HomeserverInfo, Request, Response};

use crate::Result;

/// `GET /.well-known/matrix/client` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#getwell-knownmatrixclient
pub async fn get_discovery_information(
    State(app): State<Arc<App>>,
    _: Request,
) -> Result<Response> {
    let base_url = match &app.config.discovery.base_url {
        Some(base_url) => base_url.to_string(),
        None => format!("https://{}", &app.config.server_name),
    };

    Ok(Response::new(HomeserverInfo::new(base_url)))
}
