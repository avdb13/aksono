use std::convert::Infallible;

use http::StatusCode;
use ruma::{
    api::client::{
        error::{Error as RumaError, ErrorBody, ErrorKind},
        uiaa::{UiaaInfo, UiaaResponse},
    },
    OwnedServerName,
};
use tracing::{error, warn};

use crate::Ra;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}: {1}")]
    BadRequest(ErrorKind, &'static str),
}
