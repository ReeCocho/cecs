use std::any::TypeId;

use crate::component::Component;

pub mod archetypes;
pub mod buffer;

/// Describes a set of component types.
#[derive(Debug, Clone)]
pub struct Archetype {
    ids: Vec<TypeId>,
}

impl Default for Archetype {
    fn default() -> Self {
        todo!()
    }
}

impl Archetype {
    /// Returns an iterator over all of the type IDs of the components within the archetype.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<TypeId> {
        todo!()
    }

    #[inline]
    pub fn len(&self) -> usize {
        todo!()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn add_component<T: Component>(&mut self) {
        todo!()
    }

    pub fn add_component_by_id(&mut self, id: TypeId) {
        todo!()
    }

    /// Compares two archetypes and returns `true` if this archetype is a subset of the `other`.
    ///
    /// That is, every component of this archetype is contained within the `other`. This needs to
    /// be a fast operation (O(n) fast at least). The speed requirement is probably going to
    /// dictate the internal representation of the archetype.
    pub fn subset_of(&self, other: &Archetype) -> bool {
        todo!()
    }
}
