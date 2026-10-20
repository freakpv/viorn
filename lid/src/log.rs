pub mod level;
pub mod logger;
pub mod msg;
pub mod sink;
pub use level::LogLevel;
pub use logger::Logger;
pub use msg::LogMsg;
pub use sink::LogSink;
pub use sink::SlidingLogFile;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.is_allowed($crate::log::LogLevel::Error) {
            logger.log($crate::log::LogLevel::Error, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.is_allowed($crate::log::LogLevel::Warn) {
            logger.log($crate::log::LogLevel::Warn, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.is_allowed($crate::log::LogLevel::Info) {
            logger.log($crate::log::LogLevel::Info, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.is_allowed($crate::log::LogLevel::Debug) {
            logger.log($crate::log::LogLevel::Debug, format_args!($($arg)*));
        }
    }};
}
