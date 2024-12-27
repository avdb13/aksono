use std::{convert::Infallible, process::ExitCode};

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

    Ok(())
}
