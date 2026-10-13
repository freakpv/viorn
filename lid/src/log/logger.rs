use super::level::LogLevel;
use std::io;

pub struct Logger {
    max_file_size: u64,
    cur_file_size: std::sync::Mutex<u64>,
    log_file: std::sync::Mutex<std::fs::File>,
    log_level: LogLevel,
}

impl Logger {
    pub fn new(
        file_path: &std::path::Path,
        max_file_size: u64,
        log_level: LogLevel,
    ) -> io::Result<Logger> {
        use std::fs::OpenOptions;
        let log_file = OpenOptions::new().append(true).open(file_path)?;
        let meta = log_file.metadata()?;
        Ok(Logger {
            max_file_size: max_file_size,
            cur_file_size: meta.len().into(),
            log_file: log_file.into(),
            log_level: log_level,
        })
    }

    #[inline]
    pub fn allowed_log_level(&self, lvl: LogLevel) -> bool {
        return lvl <= self.log_level;
    }

    pub fn write_msg(&self, lvl: LogLevel, msg_args: std::fmt::Arguments<'_>) {
        use std::io::Write;
        let mut msg = std::vec::Vec::with_capacity(512);
        write!(msg, "{}: ", lvl.as_str()).unwrap();
        write!(msg, "{}: ", msg_args).unwrap();

        let mut cur_size = self.cur_file_size.lock().unwrap();
        *cur_size += msg.len() as u64;

        let mut log_file = self.log_file.lock().unwrap();

        if *cur_size > self.max_file_size {
            let rem_len = std::cmp::min(100 * 1024 * 1024u64, (5 * self.max_file_size) / 100);
            unsafe {
                use std::os::fd::AsRawFd;
                libc::fallocate(
                    log_file.as_raw_fd(),
                    libc::FALLOC_FL_COLLAPSE_RANGE,
                    0,
                    rem_len as i64,
                );
            }
        }

        let _ = log_file.write(&msg);
    }
}
