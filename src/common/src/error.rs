use thiserror::Error;

pub mod api;
pub mod startup;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to start application")]
    Startup(#[from] startup::Error),

    #[error("failed to handle request")]
    Api(#[from] api::Error),
}
