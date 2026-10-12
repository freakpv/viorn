use super::level::LogLevel;
use std::io::Write;

pub struct Logger {
    _max_file_size: u64,

    log_level: LogLevel,
}

impl Logger {
    pub fn new(_file_path: &std::path::Path, max_file_size: u64, log_level: LogLevel) -> Logger {
        Logger {
            _max_file_size: max_file_size,
            log_level: log_level,
        }
    }

    #[inline]
    pub fn allowed_log_level(&self, lvl: LogLevel) -> bool {
        return lvl <= self.log_level;
    }
    pub fn write_msg(&self, lvl: LogLevel, msg_args: std::fmt::Arguments<'_>) {
        // TODO:
        std::io::stdout()
            .write(format!("{}: ", lvl.as_str()).as_bytes())
            .unwrap();
        std::io::stdout().write_fmt(msg_args).unwrap();
    }
}
