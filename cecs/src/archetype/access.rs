use std::ptr::NonNull;

use crate::{
    component::{
        filter::{ComponentAccess, ComponentFilter},
        Component,
    },
    prw_lock::{PrwReadHandle, PrwWriteHandle},
};

use paste::*;
use unsafe_unwrap::*;

use super::archetypes::Archetypes;

/// A set of data buffers used to access components within a query.
pub trait DataBufferSet {
    type Filter: ComponentFilter;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    /// Determines if the provided index points to valid components
    fn is_valid(&self, idx: usize) -> bool;

    /// Fetch a filter in the set by index.
    ///
    /// # Safety
    /// No bounds checking should be performed to maximize performance. It is up to the caller to
    /// ensure the index is valid.
    unsafe fn fetch(&mut self, idx: usize) -> Self::Filter;
}

/// A way to access a data buffer belonging to an archetype.
pub trait DataBufferAccess: Default {
    /// Type of component held in the buffer.
    type Component: Component;

    /// How the components must be accessed.
    type ComponentAccess: ComponentAccess;

    /// Access the Nth storage buffer of the associated component type within the `Archetypes`
    /// container.
    fn new(archetypes: &Archetypes, idx: usize) -> Self;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    /// Determines if the provided index is valid.
    fn is_valid(&self, idx: usize) -> bool;

    /// Fetch a component in the buffer by index.
    ///
    /// # Safety
    /// No bounds checking should be performed to maximize performance. It is up to the caller to
    /// ensure the index is valid.
    unsafe fn fetch(&mut self, idx: usize) -> Self::ComponentAccess;
}

pub struct ReadDataBuffer<T> {
    #[allow(dead_code)]
    handle: Option<PrwReadHandle<Vec<T>>>,
    end: NonNull<T>,
    ptr: NonNull<T>,
}

pub struct WriteDataBuffer<T> {
    #[allow(dead_code)]
    handle: Option<PrwWriteHandle<Vec<T>>>,
    end: NonNull<T>,
    ptr: NonNull<T>,
}

impl<T: Component + 'static> DataBufferAccess for ReadDataBuffer<T> {
    type Component = T;
    type ComponentAccess = &'static Self::Component;

    #[inline]
    fn new(archetypes: &Archetypes, index: usize) -> Self {
        let handle = archetypes
            .get_component_buffers::<Self::Component>()
            .expect("Requested non existant storage")
            .get(index);
        let end = unsafe { handle.as_ptr().add(handle.len()) };
        let ptr = handle.as_ptr();

        Self {
            handle: Some(handle),
            end: if end.is_null() {
                unsafe { NonNull::new_unchecked(1 as *mut T) }
            } else {
                unsafe { NonNull::new_unchecked(end as *mut T) }
            },
            ptr: if ptr.is_null() {
                unsafe { NonNull::new_unchecked(1 as *mut T) }
            } else {
                unsafe { NonNull::new_unchecked(ptr as *mut T) }
            },
        }
    }

    #[inline]
    fn len(&self) -> usize {
        unsafe { self.end.as_ptr().offset_from(self.ptr.as_ptr()) as usize }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.end == self.ptr
    }

    #[inline(always)]
    fn is_valid(&self, idx: usize) -> bool {
        unsafe { idx < self.end.as_ptr().offset_from(self.ptr.as_ptr()) as usize }
    }

    #[inline(always)]
    unsafe fn fetch(&mut self, idx: usize) -> Self::ComponentAccess {
        self.ptr.as_ptr().add(idx).as_ref().unsafe_unwrap()
    }
}

impl<T> Default for ReadDataBuffer<T> {
    #[inline]
    fn default() -> Self {
        Self {
            handle: None,
            end: unsafe { NonNull::new_unchecked(1 as *mut T) },
            ptr: unsafe { NonNull::new_unchecked(1 as *mut T) },
        }
    }
}

unsafe impl<T> Send for ReadDataBuffer<T> {}

unsafe impl<T> Sync for ReadDataBuffer<T> {}

impl<T: Component + 'static> DataBufferAccess for WriteDataBuffer<T> {
    type Component = T;
    type ComponentAccess = &'static mut T;

    #[inline]
    fn new(archetypes: &Archetypes, index: usize) -> Self {
        let mut handle = archetypes
            .get_component_buffers::<Self::Component>()
            .expect("Requested non existant storage")
            .get_mut(index);
        let end = unsafe { handle.as_ptr().add(handle.len()) };
        let ptr = handle.as_mut_ptr();

        Self {
            handle: Some(handle),
            end: if end.is_null() {
                unsafe { NonNull::new_unchecked(1 as *mut T) }
            } else {
                unsafe { NonNull::new_unchecked(end as *mut T) }
            },
            ptr: if ptr.is_null() {
                unsafe { NonNull::new_unchecked(1 as *mut T) }
            } else {
                unsafe { NonNull::new_unchecked(ptr) }
            },
        }
    }

    #[inline]
    fn len(&self) -> usize {
        unsafe { self.end.as_ptr().offset_from(self.ptr.as_ptr()) as usize }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.end == self.ptr
    }

    #[inline(always)]
    fn is_valid(&self, idx: usize) -> bool {
        unsafe { idx < self.end.as_ptr().offset_from(self.ptr.as_ptr()) as usize }
    }

    #[inline(always)]
    unsafe fn fetch(&mut self, idx: usize) -> Self::ComponentAccess {
        self.ptr.as_ptr().add(idx).as_mut().unsafe_unwrap()
    }
}

impl<T> Default for WriteDataBuffer<T> {
    #[inline]
    fn default() -> Self {
        Self {
            handle: None,
            end: unsafe { NonNull::new_unchecked(1 as *mut T) },
            ptr: unsafe { NonNull::new_unchecked(1 as *mut T) },
        }
    }
}

unsafe impl<T> Send for WriteDataBuffer<T> {}

unsafe impl<T> Sync for WriteDataBuffer<T> {}

macro_rules! data_buffer_set_impl {
    ( $n:expr, $( $name:ident )+ ) => {
        impl<$($name: DataBufferAccess,)*> DataBufferSet for ($($name,)*) {
            type Filter = ($($name::ComponentAccess,)*);

            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[inline(always)]
            fn is_valid(&self, idx: usize) -> bool {
                self.0.is_valid(idx)
            }

            #[inline(always)]
            unsafe fn fetch(&mut self, idx: usize) -> Self::Filter {
                paste! {
                    #[allow(non_snake_case)]
                    let ($([<$name _storage>],)*) = self;
                }

                paste! { ($(
                    [<$name _storage>].fetch(idx),
                )*) }
            }
        }
    }
}

data_buffer_set_impl! { 1, A }
data_buffer_set_impl! { 2, A B }
data_buffer_set_impl! { 3, A B C }
data_buffer_set_impl! { 4, A B C D }
data_buffer_set_impl! { 5, A B C D E }
data_buffer_set_impl! { 6, A B C D E F }
data_buffer_set_impl! { 7, A B C D E F G }
data_buffer_set_impl! { 8, A B C D E F G H }
data_buffer_set_impl! { 9, A B C D E F G H I }
data_buffer_set_impl! { 10, A B C D E F G H I J }
data_buffer_set_impl! { 11, A B C D E F G H I J K }
data_buffer_set_impl! { 12, A B C D E F G H I J K L }
data_buffer_set_impl! { 13, A B C D E F G H I J K L M }
data_buffer_set_impl! { 14, A B C D E F G H I J K L M N }
data_buffer_set_impl! { 15, A B C D E F G H I J K L M N O }
data_buffer_set_impl! { 16, A B C D E F G H I J K L M N O P }
