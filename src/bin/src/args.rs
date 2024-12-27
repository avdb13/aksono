#[derive(argh::FromArgs)]
#[argh(description = "Matrix homeserver implementation in Rust")]
pub struct Args {
    /// path to configuration file
    #[argh(option, short = 'c')]
    pub config: Option<std::path::PathBuf>,
}
