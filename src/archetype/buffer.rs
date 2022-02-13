use std::any::Any;

use crate::prw_lock::{PrwLock, PrwReadHandle, PrwWriteHandle};

/// Holds lists of objects of a single type. The `Archetypes` uses these to allocate memory for
/// components and entities.
#[derive(Default)]
pub struct DataBuffers<T: Send + Sync> {
    buffers: Vec<PrwLock<Vec<T>>>,
}

pub trait GenericDataBuffers: Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Makes a new buffer and returns the index of that buffer.
    fn create(&mut self) -> usize;
}

impl<T: Send + Sync> DataBuffers<T> {
    /// Requests immutable access to a buffer within the container.
    ///
    /// # Panic
    /// Should panic if the buffer is currently being written to or if the provided buffer index
    /// is invalid.
    #[inline]
    pub fn get(&self, i: usize) -> PrwReadHandle<Vec<T>> {
        todo!()
    }

    /// Requests mutable access to a buffer within the container.
    ///
    /// # Panic
    /// Should panic if the buffer is currently being read from or written to or if the provided
    /// buffer index is invalid.
    #[inline]
    pub fn get_mut(&self, i: usize) -> PrwWriteHandle<Vec<T>> {
        todo!()
    }
}

impl<T: Send + Sync> GenericDataBuffers for DataBuffers<T> {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }

    fn create(&mut self) -> usize {
        todo!()
    }
}
