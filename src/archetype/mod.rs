use std::any::TypeId;

use crate::component::Component;

pub mod access;
pub mod archetypes;
pub mod buffer;

/// Describes a set of component types.
#[derive(Debug, Clone)]
pub struct Archetype {
    ids: Vec<TypeId>,
}

impl Default for Archetype {
    fn default() -> Self {
        // returns Archetype, with initialized vec array of 0
        Archetype { ids: Vec::new() }
    }
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

    /// Compares two archetypes and returns `true` if this archetype is a subset of the `other`.
    ///
    /// That is, every component of this archetype is contained within the `other`. This needs to
    /// be a fast operation (O(n) fast at least). The speed requirement is probably going to
    /// dictate the internal representation of the archetype.
    pub fn subset_of(&self, other: &Archetype) -> bool {
        let mut x = 0;
        let mut y = 0;
        
        let a = &self.ids; 
        let b = &other.ids;
        let first = a.len();
        let second = b.len();

        // Subset will be smaller or equal to superceding set 
        if (first > second) 
            { return false; }

        // while we can iterate through either list
        while x < first && y < second {
            // if they match, continue through both
            if a[x] == b[y] {
                x += 1;
                y += 1;
                continue;
            }

            // Since it's sorted, we continue with
            // ... the smaller of the lists
            if (a[x] < b[y]) {
                x += 1;
                continue;
            }

            if (a[x] > b[y]) {
                y += 1;
                continue;
            }
        }
        
        // return true if we traversed fully through
        // ... our Vector, indicating all our items are included
        x == first
    }
}
