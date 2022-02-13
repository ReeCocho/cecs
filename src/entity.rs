use std::num::NonZeroU32;

/// An entity represents a "thing" within the world. It is described by the components associated
/// with it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    /// Since IDs are reused, a version is kept so that we can tell the difference between two
    /// entity handles with the same ID.
    ver: NonZeroU32,
}

impl Entity {
    #[inline]
    pub fn from_raw_parts(id: u32, ver: NonZeroU32) -> Self {
        Self { id, ver }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn ver(&self) -> u32 {
        self.ver.get()
    }
}
