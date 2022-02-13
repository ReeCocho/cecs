/// The world is where entities and components are stored, and the interface used for creating or
/// destroying them.
pub struct World {}

impl Default for World {
    fn default() -> Self {
        todo!()
    }
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
