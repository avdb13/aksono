use std::sync::Arc;

use aksono_common::app::App;
use axum::extract::State;
use ruma::api::client::discovery::discover_support::{Request, Response};

use crate::Result;

/// `GET /.well-known/matrix/support` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#getwell-knownmatrixsupport
pub async fn get_support_information(State(app): State<Arc<App>>, _: Request) -> Result<Response> {
    let support_page = match &app.config.discovery.support_page {
        Some(support_page) => support_page.to_string(),
        None => format!("https://{}", &app.config.server_name),
    };

    Ok(Response::with_support_page(support_page))
}
