use std::{convert::Infallible, path::Path, process::ExitCode};

use axum::Router;
use tokio::net::TcpListener;

use tracing::info;

mod config;
mod error;

#[tokio::main]
async fn main() -> ExitCode {
    match try_main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("\n{error:?}");

            ExitCode::FAILURE
        }
    }
}

async fn try_main() -> Result<(), error::Startup> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .init();

    let config: config::Config = {
        let path = Path::new("./aksono.toml");

        let file = std::fs::read_to_string(path)
            .map_err(|error| error::Config::Read(error, path.into()))?;

        toml::from_str(&file).map_err(|error| error::Config::Parse(error, path.into()))?
    };

    info!(
        address = %config.listener.ip(),
        port = %config.listener.port(),
        "serving application"
    );

    let listener = TcpListener::bind(&*config.listener)
        .await
        .map_err(|error| error::Serve::Listener(error, config.listener.clone()))?;

    axum::serve(listener, Router::new())
        .await
        .map_err(|error| error::Serve::Listener(error, config.listener.clone()))?;

    Ok(())
}
