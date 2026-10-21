pub mod bitmask;
pub mod subscriber;

pub use bitmask::signal_description;
pub use bitmask::Bitmask;
pub use subscriber::subscribe;
pub use subscriber::Subscriber;

// (1u32..u32::BITS).contains(<sig>) is currently not usable in const context and
// thus we need this helper function for the below compile time checks.
// The received signals will be packed as bits in u32 but the 0 value is special.
// It's used for marking that there are no pending signals.
const fn check_signal_mask(signal: i32) -> u32 {
    assert!(signal >= 1 && signal < u32::BITS as i32);
    signal as u32
}

pub const SIGHUP: u32 = check_signal_mask(libc::SIGHUP);
pub const SIGINT: u32 = check_signal_mask(libc::SIGINT);
pub const SIGQUIT: u32 = check_signal_mask(libc::SIGQUIT);
pub const SIGILL: u32 = check_signal_mask(libc::SIGILL);
pub const SIGTRAP: u32 = check_signal_mask(libc::SIGTRAP);
pub const SIGABRT: u32 = check_signal_mask(libc::SIGABRT);
pub const SIGBUS: u32 = check_signal_mask(libc::SIGBUS);
pub const SIGFPE: u32 = check_signal_mask(libc::SIGFPE);
pub const SIGUSR1: u32 = check_signal_mask(libc::SIGUSR1);
pub const SIGSEGV: u32 = check_signal_mask(libc::SIGSEGV);
pub const SIGUSR2: u32 = check_signal_mask(libc::SIGUSR2);
pub const SIGPIPE: u32 = check_signal_mask(libc::SIGPIPE);
pub const SIGALRM: u32 = check_signal_mask(libc::SIGALRM);
pub const SIGTERM: u32 = check_signal_mask(libc::SIGTERM);
pub const SIGCHLD: u32 = check_signal_mask(libc::SIGCHLD);
pub const SIGCONT: u32 = check_signal_mask(libc::SIGCONT);
