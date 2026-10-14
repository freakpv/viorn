use super::level::LogLevel;
use super::log_file::LogFile;
use crate::time;
use std::io;
use std::sync::mpsc;
use std::vec::Vec;

thread_local! {
    static THREAD_ID: u32 = unsafe { libc::gettid() as u32 };
}

const MAX_REM_FILE_SIZE: u64 = 100 * 1024 * 1024u64;
const LOG_QUEUE_SIZE: usize = 1024;

pub struct Logger {
    log_thread: std::thread::JoinHandle<()>,
    msg_sender: mpsc::SyncSender<Vec<u8>>,
    process_id: u32,
    log_level: LogLevel,
}

impl Logger {
    pub fn new(
        file_path: &std::path::Path,
        max_file_size: u64,
        log_level: LogLevel,
    ) -> io::Result<Logger> {
        let rem_file_size = std::cmp::min(MAX_REM_FILE_SIZE, (5 * max_file_size) / 100);
        let log_file = LogFile::open(file_path, max_file_size, rem_file_size)?;
        let (msg_sender, msg_receiver) = mpsc::sync_channel(LOG_QUEUE_SIZE);
        let log_thread = std::thread::Builder::new()
            .name("lid_logger".to_string())
            .spawn(move || {
                Self::process_msgs(msg_receiver, log_file);
            })?;
        Ok(Logger {
            log_thread: log_thread,
            msg_sender: msg_sender,
            process_id: std::process::id(),
            log_level: log_level,
        })
    }

    #[inline]
    pub fn allowed_log_level(&self, lvl: LogLevel) -> bool {
        return lvl <= self.log_level;
    }

    pub fn write_msg(&self, lvl: LogLevel, msg_args: std::fmt::Arguments<'_>) {
        use std::io::Write;
        // TODO: Allocator
        let mut msg = std::vec::Vec::with_capacity(512);
        if let Ok(dt) = time::UtcDatetime::now() {
            write!(
                msg,
                "{} {} {} {}: {}",
                dt,
                self.process_id,
                THREAD_ID.with(|id| *id),
                lvl.as_str(),
                msg_args
            )
            .unwrap();
        }
        // TODO: Count dropped
        let _ = self.msg_sender.try_send(msg);
    }

    fn process_msgs(msg_receiver: mpsc::Receiver<Vec<u8>>, mut log_file: LogFile) {
        while let Ok(msg) = msg_receiver.recv() {
            // TODO: Count the failures
            let _ = log_file.append(&msg);
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        // TODO:
        // drop(self.msg_sender);
        self.log_thread.join().unwrap();
    }
}
