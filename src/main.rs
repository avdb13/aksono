use std::{convert::Infallible, process::ExitCode};

mod config;

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("\n{error:?}");

            ExitCode::FAILURE
        }
    }
}

fn try_main() -> Result<(), Infallible> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .init();

    let _config: config::Config = {
        let file = std::fs::read_to_string("./aksono.toml")
            .map_err(|_error| todo!())
            .unwrap();

        toml::from_str(&file).map_err(|_error| todo!()).unwrap()
    };

    Ok(())
}
