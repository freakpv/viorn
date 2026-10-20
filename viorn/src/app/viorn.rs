use super::config::Config;
use anyhow::Context;
use std::path::Path;
use std::thread;

const LOG_QUEUE_SIZE: usize = 1024;
const MAX_REM_FILE_SIZE: u64 = 100 * 1024 * 1024u64;

pub struct Viorn {
    logger: lid::log::Logger,
    sig_sub: lid::signal::Subscriber,
    queues: Vec<u32>,
}

impl Viorn {
    pub fn new<P: AsRef<Path>>(config_path: P) -> anyhow::Result<Viorn> {
        let sig_sub = lid::signal::subscribe(&[lid::signal::SIGINT, lid::signal::SIGTERM])?;
        let config = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config {}", config_path.as_ref().display()))
            .and_then(|config_data| Config::load(&config_data))?;
        let log_level = config
            .logging
            .log_level
            .parse()
            .map_err(anyhow::Error::msg)?;
        let max_file_size = config.logging.max_size_mb as u64 * 1024 * 1024;
        let rem_file_size = std::cmp::min(MAX_REM_FILE_SIZE, (5 * max_file_size) / 100);
        let sliding_log = lid::log::SlidingLogFile::open(
            config.logging.file_path.as_path(),
            max_file_size,
            rem_file_size,
        )?;
        let logger = lid::log::Logger::new(
            log_level,
            LOG_QUEUE_SIZE,
            [Box::new(sliding_log) as Box<dyn lid::log::LogSink>],
        );

        Ok(Viorn {
            logger: logger,
            sig_sub: sig_sub,
            queues: config.filter.queues,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let logger = &self.logger;
        let sig_sub = &self.sig_sub;
        let cnt_runners = self.queues.len();
        thread::scope(|s| {
            let mut runners = vec![];
            runners.reserve_exact(cnt_runners + 2);
            runners.push(Self::spawn_thread(s, "viorn-logger".into(), || {
                logger.run()
            }));
            // TODO: Implement .await later to integrate it with the rest of the system
            runners.push(Self::spawn_thread(s, "viorn-stopper".into(), || {
                let sig = sig_sub.wait();
                if std::matches!(sig, lid::signal::SIGINT | lid::signal::SIGTERM) {
                    logger.stop();
                }
            }));
            for i in 0..cnt_runners {
                runners.push(Self::spawn_thread(s, "viorn-runner".into(), move || {
                    // TODO
                    lid::log_info!(logger, "This is thread{}\n", i);
                }));
            }
        });
        Ok(())
    }

    fn spawn_thread<'scope, 'env, F>(
        scope: &'scope thread::Scope<'scope, 'env>,
        name: String,
        func: F,
    ) -> std::io::Result<thread::ScopedJoinHandle<'scope, ()>>
    where
        F: FnOnce() + Send + 'scope,
    {
        thread::Builder::new().name(name).spawn_scoped(scope, func)
    }
}
