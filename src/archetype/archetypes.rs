use std::{any::TypeId, collections::HashMap};

use crate::{component::Component, entity::Entity};

use super::{
    buffer::{DataBuffers, GenericDataBuffers},
    Archetype,
};

/// Holds all of the archetype containers used in a world.
pub struct Archetypes {
    /// All currently registered archetype descriptors.
    archetype_descriptors: Vec<ArchetypeDescriptor>,
    /// Maps an archetype to the ID of the descriptor that describes its data.
    to_archetype_descriptor: HashMap<Archetype, ArchetypeDescriptorId>,
    /// The containers for component data.
    buffers: Vec<Box<dyn GenericDataBuffers>>,
    /// Maps the type ID of a component to the `DataBuffers` object that holds components of that
    /// type.
    to_buffers: HashMap<TypeId, DataBuffersId>,
    /// Data buffers for entities.
    entities: DataBuffers<Entity>,
}

/// Unique ID for an archetype descriptor.
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct ArchetypeDescriptorId(u32);

/// Unique ID for a `DataBuffers` object.
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct DataBuffersId(u32);

/// Describes where the data for an archetype exists within the `Archetypes` object.
pub struct ArchetypeDescriptor {
    /// The actual archetype being described.
    archetype: Archetype,
    /// A one-to-one mapping between the component type IDs within the archetype and the index
    /// of the `DataBuffers` that holds the components for this archetype.
    component_map: Vec<usize>,
    /// Index of the entity data buffer.
    entities: usize,
}

impl Default for Archetypes {
    fn default() -> Self {
        Archetypes {
            archetype_descriptors: Vec::default(),
            to_archetype_descriptor: HashMap::default(),
            buffers: Vec::default(),
            to_buffers: HashMap::default(),
            entities: DataBuffers::default(),
        }
    }
}

impl Archetypes {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new archetype descriptor to the container and return a unique ID for it.
    pub fn add_archetype(&mut self, descriptor: ArchetypeDescriptor) -> ArchetypeDescriptorId {
        let id = ArchetypeDescriptorId(self.archetype_descriptors.len() as u32);
        self.to_archetype_descriptor
            .insert(descriptor.archetype.clone(), id);
        self.archetype_descriptors.push(descriptor);
        id
    }

    /// Get a references to an archetype descriptor and its ID by the archetype it describes.
    ///
    /// Returns `None` if a descriptor matching the provided archetype doesn't exist.
    pub fn get_archetype_descriptor(
        &self,
        archetype: &Archetype,
    ) -> Option<(&ArchetypeDescriptor, ArchetypeDescriptorId)> {
        // NOTE: The `ArchetypeDescriptorId` is the index within `archetype_descriptors` where the
        // descriptor lives

        let a = &self.to_archetype_descriptor;
        // archetype is the key, see if the map contains it
        if !a.contains_key(archetype)
        {
            return None;
        }

        // get ArchetypeDescriptorID of position from the hashmap
        let vid = a.get(archetype).expect("Getting Descriptor ID");
        // unwrap the id to get the usize index, then get the reference from the vector
        let index = vid.0 as usize;
        let descriptor = self.archetype_descriptors.get(index).expect(" Extracting descriptor ");

        // otherwise return the descriptor as well as wrapper Id in the vector of archetype descriptors
        return Some( (descriptor,*vid) );
    }

    #[inline]
    pub fn get_entity_buffers(&self) -> &DataBuffers<Entity> {
        &self.entities
    }

    #[inline]
    pub fn get_entity_buffers_mut(&mut self) -> &mut DataBuffers<Entity> {
        &mut self.entities
    }

    /// Get the data buffers for a component type.
    ///
    /// Returns `None` if data buffers for the component don't exist.
    pub fn get_component_buffers<T: Component + 'static>(&self) -> Option<&DataBuffers<T>> {
        if let Some(idx) = self.to_buffers.get(&TypeId::of::<T>()) {
            self.buffers[idx.0 as usize]
                .as_any()
                .downcast_ref::<DataBuffers<T>>()
        } else {
            None
        }
    }

    /// Get mutable access to the data buffers for a component type.
    ///
    /// Returns `None` if data buffers for the component don't exist.
    pub fn get_component_buffers_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<&mut DataBuffers<T>> {
        if let Some(idx) = self.to_buffers.get(&TypeId::of::<T>()) {
            self.buffers[idx.0 as usize]
                .as_any_mut()
                .downcast_mut::<DataBuffers<T>>()
        } else {
            None
        }
    }

    /// Create data buffers for a component type. Should do nothing if data buffers for the
    /// component type already exist.
    pub fn create_component_buffers<T: Component + 'static>(&mut self) {
        // Step 1: Check to see if `self.buffers` already contains a buffer of the appropriate
        // type. If it does, do nothing.
        let t = TypeId::of::<T>();
        if self.to_buffers.contains_key(&t)
        {
            return;
        }
        
       // Step 2: Create a `DataBuffers<T>` instance and put it into `self.buffers`.
        let new_data:DataBuffers<T> = DataBuffers::default();
        let location = DataBuffersId::from( self.buffers.len() );
        self.buffers.push( Box::new(new_data) );

        // Step 3: Update `self.to_buffers` with a key of `TypeId::of<T>()` and a value of the
        // index of the newly created buffer in `self.buffers`.
        self.to_buffers.insert(t, location);
    }
}

impl ArchetypeDescriptor {
    #[inline]
    pub fn archetype(&self) -> &Archetype {
        &self.archetype
    }

    #[inline]
    pub fn component_map(&self) -> &[usize] {
        &self.component_map
    }

    #[inline]
    pub fn entities(&self) -> usize {
        self.entities
    }
}

impl From<u32> for ArchetypeDescriptorId {
    #[inline]
    fn from(item: u32) -> Self {
        ArchetypeDescriptorId(item)
    }
}

impl From<usize> for ArchetypeDescriptorId {
    #[inline]
    fn from(item: usize) -> Self {
        ArchetypeDescriptorId(item as u32)
    }
}

impl From<ArchetypeDescriptorId> for u32 {
    #[inline]
    fn from(item: ArchetypeDescriptorId) -> Self {
        item.0
    }
}

impl From<ArchetypeDescriptorId> for usize {
    #[inline]
    fn from(item: ArchetypeDescriptorId) -> Self {
        item.0 as usize
    }
}

impl From<u32> for DataBuffersId {
    #[inline]
    fn from(item: u32) -> Self {
        DataBuffersId(item)
    }
}

impl From<usize> for DataBuffersId {
    #[inline]
    fn from(item: usize) -> Self {
        DataBuffersId(item as u32)
    }
}

impl From<DataBuffersId> for u32 {
    #[inline]
    fn from(item: DataBuffersId) -> Self {
        item.0
    }
}

impl From<DataBuffersId> for usize {
    #[inline]
    fn from(item: DataBuffersId) -> Self {
        item.0 as usize
    }
}
