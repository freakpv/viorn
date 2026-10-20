use std::io;
use std::sync::atomic;

pub const SIGHUP: u32 = libc::SIGHUP as u32;
pub const SIGINT: u32 = libc::SIGINT as u32;
pub const SIGQUIT: u32 = libc::SIGQUIT as u32;
pub const SIGILL: u32 = libc::SIGILL as u32;
pub const SIGTRAP: u32 = libc::SIGTRAP as u32;
pub const SIGABRT: u32 = libc::SIGABRT as u32;
pub const SIGBUS: u32 = libc::SIGBUS as u32;
pub const SIGFPE: u32 = libc::SIGFPE as u32;
pub const SIGUSR1: u32 = libc::SIGUSR1 as u32;
pub const SIGSEGV: u32 = libc::SIGSEGV as u32;
pub const SIGUSR2: u32 = libc::SIGUSR2 as u32;
pub const SIGPIPE: u32 = libc::SIGPIPE as u32;
pub const SIGALRM: u32 = libc::SIGALRM as u32;
pub const SIGTERM: u32 = libc::SIGTERM as u32;
pub const SIGCHLD: u32 = libc::SIGCHLD as u32;
pub const SIGCONT: u32 = libc::SIGCONT as u32;

// The received signals will be packed as bits in u32 but the 0 value is special.
// It's used for marking that there are no pending signals.
crate::const_assert!((1u32..u32::BITS).contains(SIGHUP));
crate::const_assert!((1u32..u32::BITS).contains(SIGINT));
crate::const_assert!((1u32..u32::BITS).contains(SIGQUIT));
crate::const_assert!((1u32..u32::BITS).contains(SIGILL));
crate::const_assert!((1u32..u32::BITS).contains(SIGTRAP));
crate::const_assert!((1u32..u32::BITS).contains(SIGABRT));
crate::const_assert!((1u32..u32::BITS).contains(SIGBUS));
crate::const_assert!((1u32..u32::BITS).contains(SIGFPE));
crate::const_assert!((1u32..u32::BITS).contains(SIGUSR1));
crate::const_assert!((1u32..u32::BITS).contains(SIGSEGV));
crate::const_assert!((1u32..u32::BITS).contains(SIGUSR2));
crate::const_assert!((1u32..u32::BITS).contains(SIGPIPE));
crate::const_assert!((1u32..u32::BITS).contains(SIGALRM));
crate::const_assert!((1u32..u32::BITS).contains(SIGTERM));
crate::const_assert!((1u32..u32::BITS).contains(SIGCHLD));
crate::const_assert!((1u32..u32::BITS).contains(SIGCONT));

static PENDING_SIGNALS: atomic::AtomicU32 = atomic::AtomicU32::new(0);

pub struct Subscriber {
    // Contains the bits of the pending signals.
    // Note that if same signals are delivered before the subscriber reads them,
    // they will be coalesced into one. The Linux behaves the same way.
    pending: &'static atomic::AtomicU32,
}

extern "C" fn signal_handler(sig: libc::c_int) {
    let mut prev = PENDING_SIGNALS.load(atomic::Ordering::Relaxed);
    loop {
        let new = prev | (1u32 << sig);
        match PENDING_SIGNALS.compare_exchange(
            prev,
            new,
            atomic::Ordering::AcqRel,
            atomic::Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(val) => prev = val,
        }
    }
    // We don't know which subscriber is subscribed to this particular signal and thus we notify
    // all subscribers waiting for these signals.
    use crate::atomic_ext::AtomicExt;
    PENDING_SIGNALS.notify_all();
}

impl Subscriber {
    pub fn wait(&self) -> u32 {
        use crate::atomic_ext::AtomicExt;
        loop {
            // Continue waiting if the wait was interrupted but the value hasn't changed.
            if self.pending.wait(0, atomic::Ordering::Acquire) {
                return self.pending.swap(0, atomic::Ordering::Acquire);
            }
        }
    }
}

pub fn subscribe(signals: &[u32]) -> io::Result<Subscriber> {
    let mut handlers = fastvec::SmallVec::<(i32, libc::sighandler_t), 8>::new();
    let sig_hnd = signal_handler as extern "C" fn(_) as libc::sighandler_t;
    for sig in signals {
        std::assert!((1u32..u32::BITS).contains(sig));
        match unsafe { libc::signal(*sig as i32, sig_hnd) } {
            libc::SIG_ERR => {
                // Restore the signal handlers
                handlers.iter().for_each(|pair| {
                    unsafe { libc::signal(pair.0, pair.1) };
                });
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid signal number: {}", sig),
                ));
            }
            handler => handlers.push((*sig as i32, handler)),
        }
    }
    Ok(Subscriber {
        pending: &PENDING_SIGNALS,
    })
}
