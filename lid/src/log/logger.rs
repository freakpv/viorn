use super::level::LogLevel;
use super::msg::LogMsg;
use super::sink::LogSink;
use crate::time;
use std::io;
use std::sync::mpsc;
use std::sync::Mutex;

type LogSinks = fastvec::SmallVec<Box<dyn LogSink>, 4>;

pub struct Logger {
    log_sinks: Mutex<LogSinks>,
    msg_receiver: Mutex<mpsc::Receiver<LogMsg>>,
    msg_sender: mpsc::SyncSender<LogMsg>,
    process_id: u32,
    log_level: LogLevel,
}

impl Logger {
    pub fn new(
        log_level: LogLevel,
        log_queue_size: usize,
        log_sinks: impl IntoIterator<Item = Box<dyn LogSink>>,
    ) -> Logger {
        assert!(log_queue_size > 0);
        let (msg_sender, msg_receiver) = mpsc::sync_channel(log_queue_size);
        let mut temp_sinks = LogSinks::new();
        for sink in log_sinks {
            temp_sinks.push(sink);
        }
        Logger {
            log_sinks: temp_sinks.into(),
            msg_receiver: msg_receiver.into(),
            msg_sender: msg_sender,
            process_id: std::process::id(),
            log_level: log_level,
        }
    }

    #[inline]
    pub fn allowed_log_level(&self, lvl: LogLevel) -> bool {
        return lvl <= self.log_level;
    }

    pub fn log(&self, lvl: LogLevel, msg_args: std::fmt::Arguments<'_>) {
        thread_local! {
            static THREAD_ID: u32 = unsafe { libc::gettid() as u32 };
        }

        // TODO: Count dropped, etc
        let _ = time::UtcDatetime::now()
            .map(|datetime| {
                LogMsg::create(
                    datetime,
                    self.process_id,
                    THREAD_ID.with(|id| *id),
                    lvl.as_str(),
                    msg_args,
                )
            })
            .and_then(|msg| {
                self.msg_sender
                    .try_send(msg)
                    .map_err(|_| io::Error::from(io::ErrorKind::WouldBlock))
            });
    }

    pub fn run(&self) {
        let mut sinks = self.log_sinks.lock().unwrap();
        let receiver = self.msg_receiver.lock().unwrap();
        while let Ok(msg) = receiver.recv() {
            if msg.is_empty() {
                break;
            }
            // TODO: Count the failures
            for sink in sinks.iter_mut() {
                let _ = sink.log(&msg);
            }
        }
    }
}

/*
impl Drop for Logger {
    fn drop(&mut self) {
        // TODO:
        // drop(self.msg_sender);
        self.log_thread.join().unwrap();
    }
}
*/
