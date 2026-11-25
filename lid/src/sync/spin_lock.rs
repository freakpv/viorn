// The simplest possible implementation of (an inefficient) spin lock.
// The idea is this one to be used for the cases where something won't be actually used in MT
// scenario but you still need to wrap it in a Mutex/Lock to satisfy the borrow checker.
// One example for above scenario is Arena allocator which is used locally in a given function.
// std::sync::nonpoisen::Mutex is OK in size (4 bytes overhead) but the generated assembly
// is a bit bigger due to the contention handling logic and the system calls.
use super::lock_guard::{BasicLockGuard, BasicRawMutex};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

struct BasicRawSpinLock(AtomicBool);

impl BasicRawMutex for BasicRawSpinLock {
    fn lock(&self) {
        while self.0.swap(true, Ordering::AcqRel) {
            std::hint::spin_loop();
        }
    }
    fn unlock(&self) {
        self.0.store(false, Ordering::Release)
    }
}

pub struct BasicSpinLock<T: ?Sized> {
    lock: BasicRawSpinLock,
    data: UnsafeCell<T>,
}

impl<T> BasicSpinLock<T> {
    pub fn new(val: T) -> Self {
        Self {
            lock: BasicRawSpinLock(AtomicBool::new(false)),
            data: UnsafeCell::new(val),
        }
    }
    fn lock(&self) -> BasicLockGuard<'_, T, BasicRawSpinLock> {
        self.lock.lock();
        // The Mutex is locked and this ensures that the preconditions for calling
        // UnsafeCell::as_mut_unchecked are met and there will be no UB introduced.
        unsafe { BasicLockGuard::new(&self.lock, self.data.as_mut_unchecked()) }
    }
}

impl<T: Default> Default for BasicSpinLock<T> {
    fn default() -> Self {
        BasicSpinLock::new(Default::default())
    }
}

impl<T> From<T> for BasicSpinLock<T> {
    fn from(t: T) -> Self {
        BasicSpinLock::new(t)
    }
}

unsafe impl<T: ?Sized + Send> Send for BasicSpinLock<T> {}
unsafe impl<T: ?Sized + Send> Sync for BasicSpinLock<T> {}
