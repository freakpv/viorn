use std::io;
use std::sync::atomic;

pub trait AtomicExt<T> {
    fn wait(&self, old_val: T, order: atomic::Ordering) -> bool;
    fn notify_one(&self);
    fn notify_all(&self);
}

impl AtomicExt<u32> for atomic::AtomicU32 {
    fn wait(&self, old_val: u32, order: atomic::Ordering) -> bool {
        // TODO: Maybe it'll be better to check the current value against the old_val before doing
        // the "expensive" system call??? The C++ standard library even spins before the futex call
        let res = unsafe {
            libc::syscall(
                libc::SYS_futex,
                self.as_ptr(),
                libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
                old_val,
                std::ptr::null::<libc::timespec>(),
            )
        };
        if res == 0 {
            return true;
        }
        match io::Error::last_os_error().raw_os_error().unwrap() {
            libc::EWOULDBLOCK => return true, // Already different value
            libc::EINTR => return self.load(order) != old_val, // Waiting has been interrupted
            // libc::EFAULT | libc::EINVAL | libc::ETIMEDOUT
            _ => panic!("Unexpected error from futex wait"),
        }
    }
    fn notify_one(&self) {
        let res = unsafe {
            libc::syscall(
                libc::SYS_futex,
                self.as_ptr(),
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                1,
            )
        };
        std::assert!(res == 0); // May return different res only for EINVAL. This should not happen
    }
    fn notify_all(&self) {
        let res = unsafe {
            libc::syscall(
                libc::SYS_futex,
                self.as_ptr(),
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                libc::c_int::MAX,
            )
        };
        std::assert!(res == 0); // May return different res only for EINVAL. This should not happen
    }
}
