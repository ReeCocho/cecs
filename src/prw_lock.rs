use std::ops::{Deref, DerefMut};

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
pub struct PrwLock<T> {
    _remove_me: std::marker::PhantomData<T>,
}

pub struct PrwReadHandle<T> {
    _remove_me: std::marker::PhantomData<T>,
}

pub struct PrwWriteHandle<T> {
    _remove_me: std::marker::PhantomData<T>,
}

impl<T> PrwLock<T> {
    pub fn new(data: T) -> Self {
        todo!()
    }

    /// Gets read access to the data in the lock, or returns `None` if we can't access the data.
    pub fn read(&self) -> Option<PrwReadHandle<T>> {
        todo!()
    }

    /// Gets read access to the data in the lock, or returns `None` if we can't access the data.
    pub fn write(&self) -> Option<PrwWriteHandle<T>> {
        todo!()
    }
}

impl<T> Deref for PrwReadHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

unsafe impl<T> Send for PrwReadHandle<T> {}

unsafe impl<T> Sync for PrwReadHandle<T> {}

impl<T> Deref for PrwWriteHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for PrwWriteHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

unsafe impl<T> Send for PrwWriteHandle<T> {}

unsafe impl<T> Sync for PrwWriteHandle<T> {}
