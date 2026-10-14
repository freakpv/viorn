pub mod level;
pub mod log_file;
pub mod logger;
pub use level::LogLevel;
pub use logger::Logger;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.allowed_log_level($crate::log::LogLevel::Error) {
            logger.write_msg($crate::log::LogLevel::Error, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.allowed_log_level($crate::log::LogLevel::Warn) {
            logger.write_msg($crate::log::LogLevel::Warn, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.allowed_log_level($crate::log::LogLevel::Info) {
            logger.write_msg($crate::log::LogLevel::Info, format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {{
        let logger = &$logger;
        if logger.allowed_log_level($crate::log::LogLevel::Debug) {
            logger.write_msg($crate::log::LogLevel::Debug, format_args!($($arg)*));
        }
    }};
}
