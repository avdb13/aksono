use std::{fmt::Debug, ops::Deref};

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Path},
    http::StatusCode,
    response::IntoResponse,
    RequestExt as _, RequestPartsExt as _,
};
use bytes::BytesMut;
use http_body_util::BodyExt as _;
use ruma::api::{client, IncomingRequest, OutgoingResponse};
use ruma_handler::RumaHandler;

use crate::error;

mod ruma_handler;

pub struct Router<S = ()>(axum::Router<S>);

pub trait RouterExt<S> {
    fn route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T, S>,
        T: 'static;
}

#[allow(clippy::new_without_default)]
impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self(axum::Router::new())
    }

    pub fn into_inner(self) -> axum::Router<S> {
        self.0
    }
}

impl<S> Deref for Router<S> {
    type Target = axum::Router<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> RouterExt<S> for Router<S> {
    fn route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T, S>,
        T: 'static,
    {
        Router(handler.add_to_router(self.0))
    }
}

pub(super) struct Incoming<T>(T);

#[derive(Clone)]
pub(super) struct Outgoing<T>(T);

#[async_trait]
impl<S, T> FromRequest<S> for Incoming<T>
where
    T: IncomingRequest,
    S: Debug,
{
    type Rejection = error::api::Error;

    async fn from_request(
        request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = request.with_limited_body().into_parts();

        let body = match body.collect().await {
            Ok(body) => body.to_bytes(),
            Err(_) => todo!(),
        };

        let path: Path<Vec<String>> = match parts.extract().await {
            Ok(path) => path,
            Err(_) => todo!(),
        };

        let inner = match T::try_from_http_request(
            {
                let mut request = axum::extract::Request::builder()
                    .uri(parts.uri)
                    .method(parts.method);

                *request.headers_mut().unwrap() = parts.headers;

                request.body(body).unwrap()
            },
            &path,
        ) {
            Ok(inner) => inner,
            Err(_) => todo!(),
        };

        Ok(Self(inner))
    }
}

impl<T: OutgoingResponse> IntoResponse for Outgoing<T> {
    fn into_response(self) -> axum::response::Response {
        match self.0.try_into_http_response::<BytesMut>() {
            Ok(res) => res.map(BytesMut::freeze).map(Body::from).into_response(),
            Err(_) => todo!(),
        }
    }
}

impl IntoResponse for error::api::Error {
    fn into_response(self) -> axum::response::Response {
        use ruma::api::client::error::ErrorKind::*;

        let (kind, status_code) = match self {
            _ => (Unknown, StatusCode::INTERNAL_SERVER_ERROR),
        };

        Outgoing(client::uiaa::UiaaResponse::MatrixError(client::Error::new(
            status_code,
            client::error::ErrorBody::Standard {
                kind,
                message: format!("{self}"),
            },
        )))
        .into_response()
    }
}
