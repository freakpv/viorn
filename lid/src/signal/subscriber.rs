use std::io;
use std::sync::atomic;

static PENDING_SIGNALS: atomic::AtomicU32 = atomic::AtomicU32::new(0);

pub struct Subscriber {
    // Contains the bits of the pending signals.
    // Note that if same signals are delivered before the subscriber reads them,
    // they will be coalesced into one. The Linux behaves the same way.
    pending: &'static atomic::AtomicU32,
    // The OR-ed subscribed signals
    sub_sigs: u32,
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
    // Returns a bit-mask with the received signals
    pub fn wait(&self) -> super::Bitmask {
        use crate::atomic_ext::AtomicExt;
        loop {
            // Continue waiting if the wait was interrupted but the value hasn't changed.
            if !self.pending.wait(0, atomic::Ordering::Acquire) {
                continue;
            }
            let recv_sigs = self.pending.try_update(
                atomic::Ordering::AcqRel,
                atomic::Ordering::Relaxed,
                |recv_sigs| match recv_sigs & self.sub_sigs {
                    0u32 => return None,
                    _ => return Some(recv_sigs & !self.sub_sigs),
                },
            );
            if let Ok(rs) = recv_sigs {
                return super::Bitmask {
                    bits: rs & self.sub_sigs,
                };
            }
        }
    }
}

pub fn subscribe(signals: &[u32]) -> io::Result<Subscriber> {
    let mut handlers = fastvec::SmallVec::<(i32, libc::sighandler_t), 8>::new();
    let sig_hnd = signal_handler as extern "C" fn(_) as libc::sighandler_t;
    for sig in signals {
        assert!((1..u32::BITS).contains(sig));
        match subscribe_signal(*sig as i32, sig_hnd) {
            libc::SIG_ERR => {
                // Restore the signal handlers
                handlers.iter().for_each(|pair| {
                    subscribe_signal(pair.0, pair.1);
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
        sub_sigs: handlers
            .iter()
            .fold(0u32, |mask, &pair| mask | (1u32 << pair.0)),
    })
}

#[inline]
fn subscribe_signal(sig: i32, hnd: libc::sighandler_t) -> libc::sighandler_t {
    unsafe { libc::signal(sig, hnd) }
}
