use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ruma::api::client::{
    error::{ErrorBody, ErrorKind},
    uiaa::UiaaResponse,
};

use crate::{error::Error, router::outgoing::Outgoing};

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use ErrorKind::*;

        #[allow(clippy::match_single_binding)]
        let (kind, status_code) = match self {
            _ => (Unknown, StatusCode::INTERNAL_SERVER_ERROR),
        };

        Outgoing(UiaaResponse::MatrixError(ruma::api::client::Error::new(
            status_code,
            ErrorBody::Standard {
                kind,
                message: format!("{self}"),
            },
        )))
        .into_response()
    }
}
