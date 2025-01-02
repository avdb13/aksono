use std::{marker::PhantomData, sync::Arc};

use aksono_common::app::App;
use aksono_models::entities::access_tokens::AccessToken;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestExt as _, RequestPartsExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use ruma::{
    api::{client::error::ErrorKind, AuthScheme, IncomingRequest}, OwnedDeviceId, OwnedUserId,
};

use crate::error::Error;

pub struct Sender<T> {
    pub user_id: OwnedUserId,
    pub device_id: OwnedDeviceId,
    _t: PhantomData<T>,
}

// pub enum Sender<T> {
//     User(OwnedUserId, OwnedDeviceId),
//     Appservice,
//     Server(OwnedServerName),
//     Unauthorized,
// }

#[async_trait]
impl<T> FromRequestParts<Arc<App>> for Sender<T>
where
    T: IncomingRequest,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, app: &Arc<App>) -> Result<Self, Self::Rejection> {
        #[derive(serde::Deserialize)]
        struct QueryParams {
            access_token: Option<String>,
            user_id: Option<String>,
        }

        let mut conn = app.db.get().await?;

        let metadata = T::METADATA;

        let auth_header: Option<TypedHeader<Authorization<Bearer>>> =
            parts.extract().await.unwrap();

        let QueryParams {
            access_token,
            user_id,
        } = match serde_html_form::from_str(parts.uri.query().unwrap_or_default()) {
            Ok(params) => params,
            Err(error) => {
                return Err(Error::BadRequest(
                    ErrorKind::Unknown,
                    "Failed to read query parameters",
                ));
            }
        };

        let access_token = match &auth_header {
            Some(TypedHeader(Authorization(bearer))) => Some(bearer.token()),
            None => access_token.as_deref(),
        };

        let access_token = match access_token {
            Some(access_token) => {
                let Some(access_token) = AccessToken::get(&mut conn, access_token).await? else {
                    return Err(Error::BadRequest(
                        ErrorKind::UnknownToken { soft_logout: false },
                        "Unknown access token.",
                    ));
                };

                Some((
                    access_token.user_id.parse().unwrap(),
                    OwnedDeviceId::from(access_token.device_id.unwrap()),
                ))
            }
            None => None,
        };

        match (metadata.authentication, access_token) {
            (AuthScheme::AccessToken, None) => {
                return Err(Error::BadRequest(
                    ErrorKind::MissingToken,
                    "Missing access token.",
                ));
            }
            (
                AuthScheme::AccessToken | AuthScheme::AccessTokenOptional | AuthScheme::None,
                Some((user_id, device_id)),
            ) => Ok(Sender {
                user_id,
                device_id,
                _t: PhantomData,
            }),
            (
                AuthScheme::None | AuthScheme::AppserviceToken | AuthScheme::AccessTokenOptional,
                None,
            ) => todo!(),
            (AuthScheme::ServerSignatures, Some(_)) => {
                return Err(Error::BadRequest(
                    ErrorKind::Unauthorized,
                    "Only server signatures should be used on this endpoint.",
                ));
            }
            (AuthScheme::AppserviceToken, Some(_)) => {
                return Err(Error::BadRequest(
                    ErrorKind::Unauthorized,
                    "Only appservice access tokens should be used on this \
                     endpoint.",
                ));
            }
            _ => todo!(),
        }
    }
}
