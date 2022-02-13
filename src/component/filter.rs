use std::any::TypeId;

use crate::archetype::Archetype;

/// Describes a particular way to access a subset of entities based on what components they have.
pub trait ComponentFilter {
    /// Creates an archetype which has every component within the filter.
    fn archetype() -> Archetype;

    /// Creates a list of all the access types for every component in the filter.
    fn access_descriptors() -> Vec<ComponentAccessDescriptor>;
}

/// Represents a request for access on a particular component (read or write).
pub trait ComponentAccess {
    /// Indicates this access type needs mutable access.
    const MUTABLE: bool;

    fn descriptor() -> ComponentAccessDescriptor;
}

pub struct ComponentAccessDescriptor {
    /// ID of the component being accessed.
    id: TypeId,
    /// Whether or not the access requested mutable access.
    mutable: bool,
}

impl ComponentAccessDescriptor {
    #[inline]
    pub fn id(&self) -> TypeId {
        self.id
    }

    #[inline]
    pub fn mutable(&self) -> bool {
        self.mutable
    }
}
