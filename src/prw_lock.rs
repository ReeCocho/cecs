use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

/// A panicky read-write lock.
///
/// There is a problem with the default implementation of the default Rust read-write lock:
/// https://doc.rust-lang.org/std/sync/struct.RwLock.html
///
/// The `RwLockReadGuard`'s that are returned contain a lifetime refering to the data held
/// inside of the lock. You can read about lifetimes here:
///
/// https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
///
/// Lifetimes are a somewhat advanced topic in Rust, so don't worry about it all that much. All
/// that matters is that two issues come up because of them:
///
/// 1. It makes the trait-based filters usd by systems really ugly (not a deal breaker, but kind of
///    sucks).
/// 2. You can't send them across thread boundaries (this is 100% a deal breaker).
///
/// The solution then is to create a read-write lock that doesn't contain a lifetime. Before we go
/// about tackling that, we should understand why the default `RwLock` contains a lifetime in the
/// first place.
///
/// Consider this code:
///
/// ```
/// let lock = RwLock::new(5);
///
/// let handle = lock.read().uwrap(),
///
/// std::thread::spawn(move || {
///     let x = *handle;
/// });
///
/// ```
///
/// Here's the question. What happens if `lock` is dropped before the thread completes running?
///
/// The answer is that you'll end up accessing deallocated memory and cause a panic. Thankfully,
/// Rust prevents us from using this footgun by making it impossible to send the handle across
/// the thread boundary. Unfortunately, this is exactly what we need to do in our ECS.
///
/// A good solution to this problem is to, essentially, create a reference counted lock. So, the
/// data held by the lock should be inside of something like an `Arc`, and each handle will hold
/// a copy of the `Arc` so that it only gets dropped once all handles and the lock itself are
/// dropped.
///
/// The tricky part of the implementation is going to be getting the read-write logic down. We need
/// to allow multiple threads to request reads and writes. There can be as many readers as we want,
/// but once someone requests write access, we must ensure that no one else is reading or writing.
pub struct PrwLock<T>(Arc<PrwLockInner<T>>);

struct PrwLockInner<T> {
    data: T,
    access_state: AtomicU32,
}

pub struct PrwReadHandle<T>(Arc<PrwLockInner<T>>);

pub struct PrwWriteHandle<T>(Arc<PrwLockInner<T>>);

impl<T> PrwLock<T> {
    pub fn new(data: T) -> Self {
        Self(Arc::new(PrwLockInner {
            data,
            access_state: AtomicU32::new(0),
        }))
    }

    /// Gets read access to the data in the lock.
    pub fn read(&self) -> PrwReadHandle<T> {
        // See who is accessing the PrwLock
        let access_state = self.0.access_state.fetch_add(1, Ordering::Relaxed);

        // Panic if there is a writer
        assert_ne!(access_state, u32::MAX);

        PrwReadHandle(self.0.clone())
    }

    /// Gets read access to the data in the lock.
    pub fn write(&self) -> PrwWriteHandle<T> {
        // See who is accessing the PrwLock
        let access_state = self.0.access_state.fetch_add(u32::MAX, Ordering::Relaxed);

        // Panic if there are readers or writers
        assert_eq!(access_state, 0);

        PrwWriteHandle(self.0.clone())
    }
}

impl<T> Deref for PrwReadHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<T> Drop for PrwReadHandle<T> {
    fn drop(&mut self) {
        self.0.access_state.fetch_sub(1, Ordering::Relaxed);
    }
}

unsafe impl<T> Send for PrwReadHandle<T> {}

unsafe impl<T> Sync for PrwReadHandle<T> {}

impl<T> Deref for PrwWriteHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<T> Drop for PrwWriteHandle<T> {
    fn drop(&mut self) {
        self.0.access_state.store(0, Ordering::Relaxed);
    }
}

impl<T> DerefMut for PrwWriteHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (&self.0.data as *const T as *mut T).as_mut().unwrap() }
    }
}

unsafe impl<T> Send for PrwWriteHandle<T> {}

unsafe impl<T> Sync for PrwWriteHandle<T> {}

#[cfg(test)]
mod tests {
    use super::PrwLock;

    #[test]
    fn prw_lock_test() {
        let lock = PrwLock::new(42);

        let handle1 = lock.read();
        assert_eq!(*handle1, 42);

        let handle2 = lock.read();
        assert_eq!(*handle2, 42);

        std::mem::drop(handle1);
        std::mem::drop(handle2);

        let mut handle3 = lock.write();
        assert_eq!(*handle3, 42);

        *handle3 += 27;

        assert_eq!(*handle3, 69);
    }

    #[test]
    #[should_panic]
    fn prw_lock_multiple_writers() {
        let lock = PrwLock::new(42);
        let mut _handle1 = lock.write();
        let mut _handle2 = lock.write();
    }

    #[test]
    #[should_panic]
    fn prw_lock_readers_and_writers() {
        let lock = PrwLock::new(42);
        let mut _handle1 = lock.read();
        let mut _handle2 = lock.write();
    }
}
