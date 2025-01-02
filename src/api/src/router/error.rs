use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ruma::api::client::{
    error::{ErrorBody, ErrorKind},
    uiaa::UiaaResponse,
};
use tracing::{error, warn};

use crate::{error::Error, router::outgoing::Outgoing};

impl Error {
    pub(crate) fn bad_database(message: &'static str) -> Self {
        error!(message, "Bad database");
        Self::BadDatabase(message)
    }

    pub(crate) fn bad_config(message: &'static str) -> Self {
        error!(message, "Bad config");
        Self::BadConfig(message)
    }

    pub(crate) fn to_response(&self) -> Outgoing<UiaaResponse> {
        use ErrorKind::{
            Forbidden, GuestAccessForbidden, LimitExceeded, MissingToken, NotFound, NotYetUploaded,
            ThreepidAuthFailed, ThreepidDenied, TooLarge, Unauthorized, Unknown, UnknownToken,
            Unrecognized, UserDeactivated, WrongRoomKeysVersion,
        };

        if let Self::Uiaa(uiaainfo) = self {
            return Outgoing(UiaaResponse::AuthResponse(uiaainfo.clone()));
        }

        //         if let Self::Federation(origin, error) = self {
        //             let mut error = error.clone();
        //             error.body = ErrorBody::Standard {
        //                 kind: Unknown,
        //                 message: format!("Answer from {origin}: {error}"),
        //             };
        //             return Outgoing(UiaaResponse::MatrixError(error));
        //         }

        let message = format!("{self}");

        let (kind, status_code) = match self {
            Self::BadRequest(kind, _) => (
                kind.clone(),
                match kind {
                    WrongRoomKeysVersion { .. }
                    | Forbidden { .. }
                    | GuestAccessForbidden
                    | ThreepidAuthFailed
                    | UserDeactivated
                    | ThreepidDenied => StatusCode::FORBIDDEN,
                    Unauthorized | UnknownToken { .. } | MissingToken => StatusCode::UNAUTHORIZED,
                    NotFound | Unrecognized => StatusCode::NOT_FOUND,
                    LimitExceeded { .. } => StatusCode::TOO_MANY_REQUESTS,
                    TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
                    NotYetUploaded => StatusCode::GATEWAY_TIMEOUT,
                    _ => StatusCode::BAD_REQUEST,
                },
            ),
            Self::UnsupportedRoomVersion(_) => (
                ErrorKind::UnsupportedRoomVersion,
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::Conflict(_) => (Unknown, StatusCode::CONFLICT),
            _ => (Unknown, StatusCode::INTERNAL_SERVER_ERROR),
        };

        warn!(%status_code, error = %message, "Responding with an error");

        Outgoing(UiaaResponse::MatrixError(ruma::api::client::Error::new(
            status_code,
            ErrorBody::Standard { kind, message },
        )))
    }

    /// Sanitizes public-facing errors that can leak sensitive information.
    pub(crate) fn sanitized_error(&self) -> String {
        let db_error = String::from("Database or I/O error occurred.");

        match self {
            // Self::Io { .. } => db_error,
            Self::BadConfig { .. } => db_error,
            Self::BadDatabase { .. } => db_error,
            _ => self.to_string(),
        }
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(i: std::convert::Infallible) -> Self {
        match i {}
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.to_response().into_response()
    }
}
