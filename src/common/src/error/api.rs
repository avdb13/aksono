use ruma::api::client;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0:?}: {1}")]
    BadRequest(client::error::ErrorKind, &'static str),
}
