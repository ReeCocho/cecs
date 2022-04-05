use std::any::TypeId;

use crate::component::Component;

pub mod access;
pub mod archetypes;
pub mod buffer;

/// Describes a set of component types.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Archetype {
    ids: Vec<TypeId>,
}

impl Archetype {
    /// Returns an iterator over all of the type IDs of the components within the archetype.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<TypeId> {
        self.ids.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ids.len() == 0
    }

    pub fn add_component<T: Component + 'static>(&mut self) {
        let t = TypeId::of::<T>();
        self.add_component_by_id(t);
    }

    pub fn add_component_by_id(&mut self, id: TypeId) {
        self.ids.push(id);
        self.ids.sort_unstable();
    }

    /// Returns `true` if `self` contains any of the same types as `other`.
    pub fn any_of(&self, other: &Archetype) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        let mut self_i = 0;
        let mut other_i = 0;

        while self_i != self.ids.len() && other_i != other.ids.len() {
            let self_id = self.ids[self_i];
            let other_id = other.ids[other_i];

            if self_id == other_id {
                return true;
            } else if self_id < other_id {
                self_i += 1;
            } else {
                other_i += 1;
            }
        }

        false
    }

    /// Compares two archetypes and returns `true` if this archetype is a subset of the `other`.
    ///
    /// That is, every component of this archetype is contained within the `other`. This needs to
    /// be a fast operation (O(n) fast at least). The speed requirement is probably going to
    /// dictate the internal representation of the archetype.
    pub fn subset_of(&self, other: &Archetype) -> bool {
        if self.ids.is_empty() {
            return true;
        }

        let mut i = 0;
        for ty in &other.ids {
            if *ty == self.ids[i] {
                i += 1;
                if i == self.ids.len() {
                    return true;
                }
            }
        }

        false
    }
}
