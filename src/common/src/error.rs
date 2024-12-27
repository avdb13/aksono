use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to start application")]
    Startup(#[from] Startup),
}

#[derive(Error, Debug)]
pub enum Startup {
    #[error("failed to load configuration")]
    Config(#[from] Config),

    #[error("failed to serve requests")]
    Serve(#[from] Serve),
}

#[derive(Error, Debug)]
pub enum Config {
    #[error("failed to find configuration file")]
    Search,

    #[error("failed to read configuration file {1:?}")]
    Read(#[source] std::io::Error, std::path::PathBuf),

    #[error("failed to parse configuration file {1:?}")]
    Parse(#[source] toml::de::Error, std::path::PathBuf),
}

#[derive(Error, Debug)]
pub enum Serve {
    #[error("failed to run request listener on {1}")]
    Listener(#[source] std::io::Error, crate::config::Listener),
}
