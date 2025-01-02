use std::sync::Arc;

use aksono_common::{app::App, utils};
use aksono_models::entities::{
    account_data::AccountData, devices::Device, profiles::Profile,
    uiaa_credentials::UiaaCredentials, uiaa_sessions::UiaaSession, users::User,
};
use axum::extract::State;
use ruma::{
    api::client::{
        account::register::{
            v3::{Request, Response},
            LoginType, RegistrationKind,
        },
        error::ErrorKind,
        uiaa::{AuthData, AuthFlow, AuthType, Password, UiaaInfo, UserIdentifier},
    },
    CanonicalJsonObject, DeviceId, UserId,
};

use crate::{error::Error, Result};

mod available;

pub use available::*;

/// `POST /_matrix/client/*/register` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3register
pub async fn register(State(app): State<Arc<App>>, request: Request) -> Result<Response> {
    let mut conn = app.db.get().await?;

    if app.registration_disabled(request.kind.clone()) {
        return Err(Error::BadRequest(
            ErrorKind::forbidden(),
            "Registration has been disabled.",
        ));
    }

    let user_id = match request.username {
        Some(localpart) if request.kind == RegistrationKind::User => {
            match UserId::parse_with_server_name(localpart, &app.config.server.name) {
                Ok(user_id) if !user_id.is_historical() => {
                    if (User::get(&mut conn, &user_id).await?).is_some() {
                        return Err(Error::BadRequest(
                            ErrorKind::UserInUse,
                            "The desired user ID is already taken.",
                        ));
                    }

                    user_id
                }
                _ => {
                    return Err(Error::BadRequest(
                        ErrorKind::InvalidUsername,
                        "The desired user ID is invalid.",
                    ));
                }
            }
        }
        _ => loop {
            if let Ok(user_id) =
                UserId::parse_with_server_name(utils::secure_rand_str(8), &app.config.server.name)
            {
                let Some(_) = User::get(&mut conn, &user_id).await? else {
                    break user_id;
                };
            }
        },
    };

    // UIAA
    let mut uiaainfo = UiaaInfo::new(vec![AuthFlow::new(vec![AuthType::Dummy])], Box::default());
    let skip_auth = request.kind == RegistrationKind::Guest;

    if let Some(auth) = &request.auth.filter(|_| !skip_auth) {
        if let (false, uiaainfo) = crate::c2s::try_auth(app.clone(), auth, &uiaainfo).await? {
            return Err(Error::Uiaa(uiaainfo));
        }
    } else {
        let session = utils::secure_rand_str(32);
        uiaainfo.session = Some(session.clone());

        UiaaSession::create(&mut conn, &session, CanonicalJsonObject::new()).await?;

        return Err(Error::Uiaa(uiaainfo));
    }

    let password_hash = match (&request.kind, &request.login_type) {
        (RegistrationKind::Guest, _) | (_, Some(LoginType::ApplicationService)) => None,
        _ => {
            match utils::hash_password(&request.password.ok_or_else(|| {
                Error::BadRequest(
                    ErrorKind::InvalidParam,
                    "Password can only be empty for guests and appservices.",
                )
            })?) {
                Ok(password_hash) => Some(password_hash),
                Err(_error) => {
                    return Err(Error::BadRequest(
                        ErrorKind::Unknown,
                        "Password could not be hashed.",
                    ))
                }
            }
        }
    };

    User::create(
        &mut conn,
        &user_id,
        password_hash.map(|s| s.to_string()).as_deref(),
        request.kind == RegistrationKind::Guest,
    )
    .await?;

    // Default to pretty displayname
    let displayname = user_id.localpart().to_owned();

    Profile::create(
        &mut conn,
        Profile {
            id: user_id.to_string(),
            displayname: Some(displayname.clone()),
            avatar_url: None,
        },
    )
    .await?;

    let content = ruma::events::push_rules::PushRulesEventContent {
        global: ruma::push::Ruleset::server_default(&user_id),
    };

    // Initial account data
    AccountData::create(&mut conn, &user_id, &content, None).await?;

    // Inhibit login does not work for guests
    if request.kind != RegistrationKind::Guest && request.inhibit_login {
        return Ok(Response {
            access_token: None,
            user_id,
            device_id: None,
            refresh_token: None,
            expires_in: None,
        });
    }

    // TODO: given device ID must not be the same as a cross-signing key ID.
    let device_id = match &request.device_id {
        Some(device_id) => device_id.to_owned(),
        None => DeviceId::new(),
    };

    // Generate new token for the device
    let access_token = utils::secure_rand_str(32);

    // TODO: bind to access token

    // Create device for this account
    Device::create(
        &mut conn,
        &device_id,
        &user_id,
        request.initial_device_display_name.as_deref(),
    )
    .await?;

    // info!(%user_id, "New user registered on this server");
    // if body.appservice_info.is_none() && !is_guest {
    //     services()
    //         .admin
    //         .send_message(RoomMessageEventContent::notice_plain(format!(
    //             "New user {user_id} registered on this server."
    //         )));
    // }

    // // If this is the first real user, grant them admin privileges
    // // Note: the server user, @grapevine:servername, is generated first
    // if !is_guest {
    //     if let Some(admin_room) = services().admin.get_admin_room()? {
    //         if services()
    //             .rooms
    //             .state_cache
    //             .room_joined_count(&admin_room)?
    //             == Some(1)
    //         {
    //             services()
    //                 .admin
    //                 .make_user_admin(&user_id, displayname)
    //                 .await?;

    //             warn!(
    //                 %user_id,
    //                 "Granting admin privileges to the first user",
    //             );
    //         }
    //     }
    // }

    Ok(Response {
        access_token: Some(access_token),
        user_id,
        device_id: Some(device_id),
        refresh_token: None,
        expires_in: None,
    })
}

pub async fn try_auth(
    app: Arc<App>,
    auth: &AuthData,
    uiaainfo: &UiaaInfo,
) -> Result<(bool, UiaaInfo)> {
    let mut conn = app.db.get().await?;

    let uiaainfo = match auth.session() {
        Some(session_id) => UiaaInfo {
            completed: match UiaaCredentials::find_by_session(&mut conn, session_id).await? {
                Some(v) => v
                    .iter()
                    .map(|c| AuthType::from(c.stage_type.clone()))
                    .collect(),
                None => Vec::new(),
            },
            ..uiaainfo.to_owned()
        },
        None => uiaainfo.to_owned(),
    };

    let session_id = uiaainfo
        .session
        .unwrap_or_else(|| utils::secure_rand_str(32));

    let mut uiaainfo = UiaaInfo {
        session: Some(session_id.clone()),
        ..uiaainfo
    };

    let completed = match auth {
        // Find out what the user completed
        AuthData::Password(Password {
            identifier,
            password,
            ..
        }) => {
            let UserIdentifier::UserIdOrLocalpart(username) = identifier else {
                return Err(Error::BadRequest(
                    ErrorKind::Unrecognized,
                    "Identifier type not recognized.",
                ));
            };

            let user_id = UserId::parse_with_server_name(username.clone(), &app.config.server.name)
                .map_err(|_| Error::BadRequest(ErrorKind::InvalidParam, "User ID is invalid."))?;

            let Some(user) = User::get(&mut conn, &user_id).await? else {
                todo!()
            };

            // Check if password is correct
            let Some(password_hash) = &user.password_hash else {
                todo!()
            };

            if !utils::verify_password(password_hash, password) {
                return Ok((
                    false,
                    UiaaInfo {
                        auth_error: Some(ruma::api::client::error::StandardErrorBody {
                            kind: ErrorKind::forbidden(),
                            message: "Invalid username or password.".to_owned(),
                        }),
                        ..uiaainfo
                    },
                ));
            }

            // Password was correct! Let's add it to `completed`
            AuthType::Password
        }
        AuthData::RegistrationToken(_) => {
            todo!()
        }
        AuthData::Dummy(_) => AuthType::Dummy,
        kind => {
            tracing::error!(?kind, "Auth kind not supported");

            todo!()
        }
    };

    uiaainfo.completed.push(completed.clone());

    // Check if a flow now succeeds
    match uiaainfo.flows.iter().find(|flow| {
        flow.stages
            .iter()
            .all(|stage| uiaainfo.completed.contains(stage))
    }) {
        Some(_) => {
            UiaaSession::delete(&mut conn, &session_id).await?;

            Ok((true, uiaainfo))
        }
        None => {
            UiaaCredentials::create(&mut conn, &session_id, &completed, ()).await?;

            Ok((false, uiaainfo))
        }
    }
}
