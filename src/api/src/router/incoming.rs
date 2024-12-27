use std::fmt::Debug;

use axum::{
    async_trait,
    extract::{FromRequest, Path, Request},
    RequestExt as _, RequestPartsExt as _,
};
use http_body_util::BodyExt as _;
use ruma::api::IncomingRequest;

use crate::error::Error;

pub(crate) struct Incoming<T>(pub(super) T);

#[async_trait]
impl<S, T> FromRequest<S> for Incoming<T>
where
    T: IncomingRequest,
    S: Debug,
{
    type Rejection = Error;

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
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
                let mut request = Request::builder().uri(parts.uri).method(parts.method);

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
