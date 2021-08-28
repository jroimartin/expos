//! Mutex based on the Ticket Lock spin lock described in [Algorithms for
//! Scalable Synchronization on Shared-Memory Multiprocessors][ref].
//!
//! This implementation uses `Ordering::SeqCst` for all the atomic operations.
//! This has performance implications under some circumstances, but correctness
//! has been put fist.
//!
//! [ref]: http://web.mit.edu/6.173/www/currentsemester/readings/R06-scalable-synchronization-1991.pdf

#![no_std]

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Represents a mutex based on a Ticket Lock.
pub struct TicketMutex<T> {
    /// Next ticket.
    next_ticket: AtomicUsize,

    /// Ticket being served.
    now_serving: AtomicUsize,

    /// Protected data.
    data: UnsafeCell<T>,
}

impl<T> TicketMutex<T> {
    /// Returns a `TicketMutex` protecting `data`.
    pub const fn new(data: T) -> Self {
        TicketMutex {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Locks the `TicketMutex` and returns a `TicketMutexGuard` that allows
    /// exclusive access to the protected data.
    pub fn lock(&self) -> TicketMutexGuard<T> {
        // Atomically get the next ticket and increment it.
        let ticket = self.next_ticket.fetch_add(1, Ordering::SeqCst);

        // Wait until our ticket is served and return a `TicketMutexGuard`
        // for this mutex.
        while self.now_serving.load(Ordering::SeqCst) != ticket {
            core::hint::spin_loop()
        }
        TicketMutexGuard::new(self)
    }
}

unsafe impl<T: Send> Send for TicketMutex<T> {}
unsafe impl<T: Send> Sync for TicketMutex<T> {}

/// An RAII implementation of a “scoped lock” of a `TicketMutex`. When this
/// structure is dropped (falls out of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its
/// `Deref` and `DerefMut` implementations.
///
/// This structure is created by the `lock` method on `TicketMutex`.
pub struct TicketMutexGuard<'a, T> {
    /// `TicketMutex` associated with this `TicketMutexGuard`. It is used to
    /// provide access to the protected data.
    mutex: &'a TicketMutex<T>,
}

impl<'a, T> TicketMutexGuard<'a, T> {
    /// Returns a new `TicketMutexGuard` linked to a given `TicketMutex`.
    fn new(mutex: &'a TicketMutex<T>) -> Self {
        TicketMutexGuard { mutex }
    }
}

impl<T> Deref for TicketMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Returning a reference to the protected data here is safe because a
        // `TicketMutexGuard` can only exist if the mutex is locked. Meaning
        // that we have exclusive access to the critical region.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for TicketMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Returning a mutable reference to the protected data here is safe
        // because of the same reasons explained in `Deref`.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for TicketMutexGuard<'_, T> {
    fn drop(&mut self) {
        // Release the lock by incrementing the ticket being served.
        self.mutex.now_serving.fetch_add(1, Ordering::SeqCst);
    }
}
