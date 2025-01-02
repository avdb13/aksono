use aksono_common::app::App;
use aksono_common::error::startup::Error;
use aksono_common::{app, config, error};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing::info;

pub async fn setup() -> Result<Arc<App>, Error> {
    let _ = tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .try_init();

    let config: config::Config = {
        let path = Path::new(&format!(
            "{}/../../aksono.example.toml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .to_owned();

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

    let result = app.clone();

    let (tx, rx) = oneshot::channel();

    let _handle = tokio::spawn(async move {
        match TcpListener::bind(&*app.config.listener).await {
            Ok(listener) => {
                tx.send(Ok::<_, Error>(())).unwrap();

                match axum::serve(listener, aksono_api::build_routes(app.clone())).await {
                    Ok(_) => {}
                    Err(_error) => {}
                }
            }
            Err(error) => {
                let listener = app.config.listener.clone();

                tx.send(Err(error::startup::Serve::Listener(error, listener).into()))
                    .unwrap();
            }
        }
    });

    if timeout(Duration::from_millis(10), rx).await.is_err() {
        println!("did not receive value within 10 ms");
    }

    Ok(result)
}
