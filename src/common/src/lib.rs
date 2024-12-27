pub mod app;
pub mod config;
pub mod error;
pub mod router;

pub type Result<T, E = error::Error> = core::result::Result<T, E>;
