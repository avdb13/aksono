use std::{convert::Infallible, process::ExitCode};

use axum::Router;
use tokio::net::TcpListener;

use tracing::info;

mod config;

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

async fn try_main() -> Result<(), Infallible> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .init();

    let config: config::Config = {
        let file = std::fs::read_to_string("./aksono.toml")
            .map_err(|_error| todo!())
            .unwrap();

        toml::from_str(&file).map_err(|_error| todo!()).unwrap()
    };

    info!(
        address = %config.listener.ip(),
        port = %config.listener.port(),
        "serving application"
    );

    let listener = TcpListener::bind(&*config.listener)
        .await
        .map_err(|_error| todo!())
        .unwrap();

    axum::serve(listener, Router::new())
        .await
        .map_err(|_error| todo!())
        .unwrap();

    Ok(())
}
