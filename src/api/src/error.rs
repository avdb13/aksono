use ruma::api::client::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0:?}: {1}")]
    BadRequest(ErrorKind, &'static str),
}
