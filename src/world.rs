use std::num::NonZeroU32;

use crate::{
    archetype::archetypes::{ArchetypeDescriptorId, Archetypes},
    component::pack::ComponentPack,
    entity::Entity,
};

/// The world is where entities and components are stored, and the interface used for creating or
/// destroying them.
#[derive(Default)]
pub struct World {
    pub(crate) archetypes: Archetypes,
    entities: Vec<EntityInfo>,
    free: Vec<u32>,
    /// Cache for newly created entity handles.
    entity_cache: Vec<Entity>,
}

struct EntityInfo {
    /// Current version of the entity.
    ver: NonZeroU32,
    /// Archetype that the entities components exist within.
    archetype: ArchetypeDescriptorId,
    /// Index within the data buffers of the archetype the components are located at.
    index: usize,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self, mut components: impl ComponentPack) -> &[Entity] {
        assert!(components.is_valid());

        // Create entity handles
        self.entity_cache.reserve(components.len());
        self.entity_cache.clear();

        for _ in 0..components.len() {
            if let Some(free) = self.free.pop() {
                self.entity_cache.push(Entity::from_raw_parts(
                    free,
                    self.entities[free as usize].ver,
                ));
            } else {
                let ver = NonZeroU32::new(1).unwrap();

                self.entities.push(EntityInfo {
                    ver,
                    archetype: ArchetypeDescriptorId::default(),
                    index: 0,
                });

                self.entity_cache
                    .push(Entity::from_raw_parts(self.entities.len() as u32 - 1, ver));
            }
        }

        // Move the components into their archetype
        let (archetype, begin) = components.move_into(&self.entity_cache, &mut self.archetypes);

        // Update the created entities archetypes
        for (i, entity) in self.entity_cache.iter().enumerate() {
            let mut info = &mut self.entities[entity.id() as usize];
            info.archetype = archetype;
            info.index = begin + i;
        }

        &self.entity_cache
    }
}
