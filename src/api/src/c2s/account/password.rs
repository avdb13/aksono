use std::sync::Arc;

use aksono_common::{app::App, utils};
use aksono_models::entities::{
    uiaa_sessions::UiaaSession,
    users::User,
};
use axum::extract::State;
use ruma::{
    api::client::{
        account::change_password::v3::{Request, Response},
        error::ErrorKind,
        uiaa::{AuthFlow, AuthType, UiaaInfo},
    },
    CanonicalJsonObject,
};
use tracing::info;

use crate::{error::Error, router::Sender, Result};

/// `POST /_matrix/client/*/account/password` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3accountpassword
pub async fn change_password(
    State(app): State<Arc<App>>,
    sender: Sender<Request>,
    request: Request,
) -> Result<Response> {
    let mut conn = app.db.get().await?;

    let mut uiaainfo = UiaaInfo::new(
        vec![AuthFlow::new(vec![AuthType::Password])],
        Box::default(),
    );

    if let Some(auth) = &request.auth {
        if let (false, uiaainfo) = crate::c2s::try_auth(app.clone(), auth, &uiaainfo).await? {
            return Err(Error::Uiaa(uiaainfo));
        }
    } else {
        let session = utils::secure_rand_str(32);
        uiaainfo.session = Some(session.clone());

        UiaaSession::create(&mut conn, &session, CanonicalJsonObject::new()).await?;

        return Err(Error::Uiaa(uiaainfo));
    }

    match utils::hash_password(&request.new_password) {
        Ok(password_hash) => {
            User::change_password(&mut conn, &sender.user_id, password_hash.as_ref()).await?;
        }
        Err(_error) => {
            return Err(Error::BadRequest(
                ErrorKind::Unknown,
                "Password could not be hashed.",
            ))
        }
    };

    if request.logout_devices {
        // Logout all devices except the current one
    }

    info!(user_id = %sender.user_id, "User changed their password");
    // services()
    //     .admin
    //     .send_message(RoomMessageEventContent::notice_plain(format!(
    //         "User {sender_user} changed their password."
    //     )));

    Ok(Response::new())
}
