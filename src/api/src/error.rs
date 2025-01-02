use diesel_async::pooled_connection::bb8;
use ruma::api::client::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "Failed to execute database query: \
         {source}"
    )]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },
    #[error("Failed to acquire database connection: {0}")]
    Pool(#[from] bb8::RunError),

    #[error("{0}")]
    BadServerResponse(&'static str),
    #[error("{0}")]
    BadConfig(&'static str),
    #[error("{0}")]
    /// Don't create this directly. Use [`Error::bad_database`] instead.
    BadDatabase(&'static str),
    #[error("uiaa: {0:?}")]
    Uiaa(ruma::api::client::uiaa::UiaaInfo),
    #[error("{0:?}: {1}")]
    BadRequest(ErrorKind, &'static str),
    // This is only needed for when a room alias already exists
    #[error("{0}")]
    Conflict(&'static str),
    #[error("{0}")]
    Extension(#[from] axum::extract::rejection::ExtensionRejection),
    #[error("{0}")]
    Path(#[from] axum::extract::rejection::PathRejection),
    #[error("{0}")]
    AdminCommand(&'static str),
    #[error("from {0}: {1}")]
    Redaction(ruma::OwnedServerName, ruma::canonical_json::RedactionError),
    #[error("unsupported room version {0}")]
    UnsupportedRoomVersion(ruma::RoomVersionId),
    #[error("{0} in {1}")]
    InconsistentRoomState(&'static str, ruma::OwnedRoomId),
}
