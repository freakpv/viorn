use std::fmt;
use std::ops::{Deref, DerefMut, Drop};

pub trait BasicRawMutex {
    fn lock(&self);
    fn unlock(&self);
}

#[must_use = "if unused the Mutex will immediately unlock"]
pub struct BasicLockGuard<'a, T: ?Sized + 'a, M: BasicRawMutex + 'a> {
    mutex: &'a M,
    data: &'a mut T,
}

impl<'a, T: ?Sized + 'a, M: BasicRawMutex + 'a> BasicLockGuard<'a, T, M> {
    /// The caller of this function is expected to have locked the given Mutex
    pub(crate) unsafe fn new(m: &'a M, d: &'a mut T) -> Self {
        BasicLockGuard { mutex: m, data: d }
    }
}

impl<'a, T: ?Sized + 'a, M: BasicRawMutex + 'a> Drop for BasicLockGuard<'a, T, M> {
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

impl<'a, T: ?Sized + 'a, M: BasicRawMutex + 'a> Deref for BasicLockGuard<'a, T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T: ?Sized + 'a, M: BasicRawMutex + 'a> DerefMut for BasicLockGuard<'a, T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

/// On platforms that use POSIX threads (commonly referred to as pthreads) there is a requirement to
/// release mutex locks on the same thread they were acquired.
/// For this reason, [`MutexGuard`] must not implement `Send` to prevent it being dropped from
/// another thread.
impl<T: ?Sized, M: BasicRawMutex> !Send for BasicLockGuard<'_, T, M> {}

/// `T` must be `Sync` for a [`MutexGuard<T>`] to be `Sync`
/// because it is possible to get a `&T` from `&MutexGuard` (via `Deref`).
unsafe impl<T: ?Sized + Sync, M: BasicRawMutex> Sync for BasicLockGuard<'_, T, M> {}

impl<T: ?Sized + fmt::Debug, M: BasicRawMutex> fmt::Debug for BasicLockGuard<'_, T, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display, M: BasicRawMutex> fmt::Display for BasicLockGuard<'_, T, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}
