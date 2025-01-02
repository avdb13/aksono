use std::sync::Arc;

use aksono_common::app::App;
use aksono_models::entities::uiaa_sessions::UiaaSession;
use axum::{
    async_trait,
    extract::{FromRequest, Path, Request},
    RequestExt as _, RequestPartsExt as _,
};
use bytes::{BufMut as _, BytesMut};
use http_body_util::{BodyExt as _, Collected};
use ruma::{
    api::{
        client::error::ErrorKind,
        IncomingRequest,
    }, CanonicalJsonValue,
};
use tracing::warn;

use crate::error::Error;

pub(crate) struct Incoming<T>(pub(super) T);

#[async_trait]
impl<T> FromRequest<Arc<App>> for Incoming<T>
where
    T: IncomingRequest,
{
    type Rejection = Error;

    async fn from_request(request: Request, app: &Arc<App>) -> Result<Self, Self::Rejection> {
        let mut conn = app.db.get().await?;

        let (mut parts, body) = request.with_limited_body().into_parts();

        let Ok(mut body) = body.collect().await.map(Collected::to_bytes) else {
            return Err(Error::BadRequest(ErrorKind::MissingToken, "Missing token."));
        };

        let path: Path<Vec<String>> = match parts.extract().await {
            Ok(path) => path,
            Err(_) => todo!(),
        };

        let mut http_request = Request::builder().uri(parts.uri).method(parts.method);
        *http_request.headers_mut().unwrap() = parts.headers;

        let mut json = serde_json::from_slice::<CanonicalJsonValue>(&body).ok();

        let session_id = json.as_ref().and_then(|json| {
            json.as_object().and_then(|auth| {
                auth.get("auth").and_then(|auth| {
                    auth.as_object().and_then(|auth| {
                        auth.get("session")
                            .and_then(|session| session.as_str().map(str::to_owned))
                    })
                })
            })
        });

        if let Some(CanonicalJsonValue::Object(json)) = &mut json {
            if let Some(session_id) = &session_id {
                if let Some(session) = UiaaSession::get(&mut conn, session_id).await? {
                    if let Ok(CanonicalJsonValue::Object(initial)) =
                        serde_json::from_str(&session.json)
                    {
                        json.extend(initial);
                    }
                }
            }

            if let Some(session_id) = &session_id {
                UiaaSession::update(&mut conn, session_id, json).await?;
            }

            let mut buf = BytesMut::new().writer();
            {
                serde_json::to_writer(&mut buf, &json).expect("value serialization can't fail");
            }
            body = buf.into_inner().freeze();
        }

        let http_request = http_request.body(body).unwrap();

        let inner = T::try_from_http_request(http_request, &path).map_err(|error| {
            warn!(
                %error,
                body = ?json,
                "Request body JSON structure is incorrect"
            );

            Error::BadRequest(ErrorKind::BadJson, "Failed to deserialize request.")
        })?;

        Ok(Self(inner))
    }
}
