#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0:?}: {1}")]
    BadRequest(ErrorKind, &'static str),
}

