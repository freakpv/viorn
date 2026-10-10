use super::config::Config;
use anyhow::Context;
use std::path::Path;

pub struct Viorn {}

impl Viorn {
    pub fn new<P: AsRef<Path>>(config_path: P) -> anyhow::Result<Viorn> {
        let _config = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config {}", config_path.as_ref().display()))
            .and_then(|config_data| Config::load(&config_data))?;

        Ok(Viorn {})
    }

    pub fn run(&mut self) {
        // TODO
    }
}
