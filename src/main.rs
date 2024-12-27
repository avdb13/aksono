use std::{path::Path, process::ExitCode};

use axum::Router;
use tokio::net::TcpListener;

use tracing::info;

mod app;
mod args;
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

    let args: args::Args = argh::from_env();

    let config: config::Config = {
        let path = args
            .config
            .unwrap_or_else(|| Path::new("./aksono.toml").to_owned());

        let file = std::fs::read_to_string(&path)
            .map_err(|error| error::Config::Read(error, path.clone()))?;

        toml::from_str(&file).map_err(|error| error::Config::Parse(error, path.clone()))?
    };

    let app = app::App::new(config);

    info!(
        address = %app.config.listener.ip(),
        port = %app.config.listener.port(),
        "serving application"
    );

    match TcpListener::bind(&*app.config.listener).await {
        Ok(listener) => match axum::serve(listener, Router::new()).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let listener = app.config.listener.clone();

                Err(error::Serve::Listener(error, listener).into())
            }
        },
        Err(error) => {
            let listener = app.config.listener.clone();

            Err(error::Serve::Listener(error, listener).into())
        }
    }
}
