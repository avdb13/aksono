use std::{path::Path, process::ExitCode, sync::Arc};

use aksono_api::build_routes;
use aksono_common::{app, config, error};
use tokio::net::TcpListener;

use tracing::info;

mod args;

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

async fn try_main() -> Result<(), error::startup::Error> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .init();

    let args: args::Args = argh::from_env();

    let config: config::Config = {
        let path = args
            .config
            .unwrap_or_else(|| Path::new("./aksono.toml").to_owned());

        let file = std::fs::read_to_string(&path)
            .map_err(|error| error::startup::Config::Read(error, path.clone()))?;

        toml::from_str(&file).map_err(|error| error::startup::Config::Parse(error, path.clone()))?
    };

    let app = Arc::new(app::App::new(config).await);

    info!(
        address = %app.config.listener.ip(),
        port = %app.config.listener.port(),
        "serving application"
    );

    match TcpListener::bind(&*app.config.listener).await {
        Ok(listener) => match axum::serve(listener, build_routes(app.clone())).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let listener = app.config.listener.clone();

                Err(error::startup::Serve::Listener(error, listener).into())
            }
        },
        Err(error) => {
            let listener = app.config.listener.clone();

            Err(error::startup::Serve::Listener(error, listener).into())
        }
    }
}
