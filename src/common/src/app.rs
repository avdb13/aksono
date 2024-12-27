use crate::config;

#[derive(Debug, Clone)]
pub struct App {
    pub config: config::Config,
}

impl App {
    pub fn new(config: config::Config) -> Self {
        Self { config }
    }
}
