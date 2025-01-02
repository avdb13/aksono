use std::sync::Arc;

use aksono_common::app::App;
use axum::extract::State;
use ruma::api::client::{
        account::{
            deactivate::v3::{Request, Response},
            ThirdPartyIdRemovalStatus,
        },
        uiaa::{AuthFlow, AuthType, UiaaInfo},
    };
use tracing::info;

use crate::{router::Sender, Result};

/// `POST /_matrix/client/*/account/deactivate` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3accountdeactivate
pub async fn deactivate_account(
    State(app): State<Arc<App>>,
    sender: Sender<Request>,
    request: Request,
) -> Result<Response> {
    // TODO: UIAA

    // TODO: Make the user leave all rooms before deactivation

    // TODO: Remove devices and mark account as deactivated

    info!(user_id = %sender.user_id, "User deactivated their account");

    Ok(Response::new(ThirdPartyIdRemovalStatus::NoSupport))
}
