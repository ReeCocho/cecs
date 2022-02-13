use crate::{
    component::{
        filter::{ComponentAccess, ComponentFilter},
        Component,
    },
    entity::Entity,
};

use super::archetypes::Archetypes;

/// A set of data buffers used to access components within a query.
pub trait DataBufferSet: Default {
    type Filter: ComponentFilter;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    /// Fetch a filter in the set by index.
    ///
    /// # Safety
    /// No bounds checking should be performed to maximize performance. It is up to the caller to
    /// ensure the index is valid.
    unsafe fn fetch(&self, idx: usize) -> (Entity, Self::Filter);
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

    /// Fetch a component in the buffer by index.
    ///
    /// # Safety
    /// No bounds checking should be performed to maximize performance. It is up to the caller to
    /// ensure the index is valid.
    unsafe fn fetch(&self, idx: usize) -> Self::ComponentAccess;
}
