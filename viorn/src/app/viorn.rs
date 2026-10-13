use super::config::Config;
use anyhow::Context;
use std::path::Path;
use std::thread;

pub struct Viorn {
    logger: lid::log::Logger,
    queues: Vec<u32>,
}

impl Viorn {
    pub fn new<P: AsRef<Path>>(config_path: P) -> anyhow::Result<Viorn> {
        let config = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config {}", config_path.as_ref().display()))
            .and_then(|config_data| Config::load(&config_data))?;

        let logger = lid::log::Logger::new(
            config.logging.file_path.as_path(),
            config.logging.max_size_mb as u64 * 1024 * 1024,
            config
                .logging
                .log_level
                .parse()
                .map_err(anyhow::Error::msg)?,
        )?;

        Ok(Viorn {
            logger: logger,
            queues: config.filter.queues,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let logger = &self.logger;
        let cnt_runners = self.queues.len();
        thread::scope(|s| {
            let mut runners = vec![];
            runners.reserve_exact(cnt_runners);
            for i in 0..cnt_runners {
                runners.push(s.spawn(move || {
                    // TODO
                    lid::log_info!(logger, "This is thread{}\n", i);
                }));
            }
        });
        Ok(())
    }
}
