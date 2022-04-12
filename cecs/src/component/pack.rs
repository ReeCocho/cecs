use crate::{
    archetype::{
        archetypes::{ArchetypeDescriptor, ArchetypeDescriptorId, Archetypes},
        buffer::GenericDataBuffers,
        Archetype,
    },
    component::Component,
    entity::Entity,
};
use paste::*;
use std::{any::TypeId, collections::HashMap};

pub trait ComponentPack: Send + Sync {
    fn is_valid(&self) -> bool;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn archetype(&self) -> Archetype;

    /// Moves all of the components in the pack into component storage along with their associated
    /// entities.
    ///
    /// Returns the ID of the archetype the components are contained within and the beginning
    /// index within each buffer the components were inserted at (in that order).
    ///
    /// Panics if the pack isn't valid or if the pack length and entity slice length mismatch.
    fn move_into(
        &mut self,
        entities: &[Entity],
        archetypes: &mut Archetypes,
    ) -> (ArchetypeDescriptorId, usize);
}

/// Macro to help implement the `ComponentPack` trait for tuples of component vectors.
macro_rules! component_pack_impl {
    ( $n:expr, $( $name:ident )+ ) => {
        /// Implementation for a tuple of vectors of components.
        impl<$($name: Component + 'static, )*> ComponentPack for ($(Vec<$name>,)*) {
            #[inline]
            fn is_valid(&self) -> bool {
                paste! {
                    #[allow(non_snake_case)]
                    let ($([<$name _ref>],)*) = self;
                }

                let len = self.0.len();
                paste! {$(
                    if [<$name _ref>].len() != len {
                        return false;
                    }
                )*}

                true
            }

            #[inline]
            fn len(&self) -> usize {
                assert!(self.is_valid());
                self.0.len()
            }

            #[inline]
            fn is_empty(&self) -> bool {
                assert!(self.is_valid());
                self.0.is_empty()
            }

            #[inline]
            fn archetype(&self) -> Archetype {
                let mut archetype = Archetype::default();
                $(
                    archetype.add_component::<$name>();
                )*
                archetype
            }

            fn move_into(
                &mut self,
                entities: &[Entity],
                archetypes: &mut Archetypes,
            ) -> (ArchetypeDescriptorId, usize)
            {
                assert!(self.is_valid());
                assert!(entities.len() >= self.len());

                // Get the archetype descriptor for the pack
                let archetype = self.archetype();
                let (descriptor, index) = if let Some(descriptor) = archetypes.get_archetype_descriptor(&archetype) {
                    descriptor
                } else {
                    // Archetype descriptor doesn't exist, so we need to make one
                    let mut descriptor = ArchetypeDescriptor {
                        archetype: archetype.clone(),
                        map: HashMap::with_capacity($n),
                        entities: 0,
                    };

                    // Initialize map values
                    $(
                        let storage = if let Some(storage) = archetypes
                            .get_component_buffers_mut::<$name>()
                        {
                            storage
                        } else {
                            archetypes.create_component_buffers::<$name>();
                            // Safe to unwrap since we just created it
                            archetypes.get_component_buffers_mut::<$name>().unwrap()
                        };
                        descriptor.map.insert(TypeId::of::<$name>(), storage.create());
                    )*

                    // Create entity buffer
                    descriptor.entities = archetypes.get_entity_buffers_mut().create();

                    // Create the archetype
                    archetypes.add_archetype(descriptor);

                    // Safe to unwrap since we just added it
                    archetypes.get_archetype_descriptor(&archetype).unwrap()
                };

                // Move all entities into the entity buffer (and store the beginning index for the entities)
                let begin_ind = {
                    let mut entity_buffer =
                        archetypes.get_entity_buffers().get_mut(descriptor.entities);
                    let begin = entity_buffer.len();
                    entity_buffer.extend_from_slice(entities);
                    begin
                };

                // Decompose the tuple
                paste! {
                    #[allow(non_snake_case)]
                    let ($([<$name _ref>],)*) = self;
                }

                // Move all components into their respective buffers
                paste!{$(
                    let ind = *descriptor.map
                        .get(&TypeId::of::<$name>())
                        .expect("Archetype map missing component type in pack.");
                    let mut buffer = archetypes
                        .get_component_buffers::<$name>()
                        .expect("Component storage missing index.").get_mut(ind);
                    for component in [<$name _ref>].drain(..) {
                        buffer.push(component);
                    }
                )*}

                // Return index of the archetype and beginning index within the buffer
                (index, begin_ind)
            }
        }
    }
}

component_pack_impl! { 1, A }
component_pack_impl! { 2, A B }
component_pack_impl! { 3, A B C }
component_pack_impl! { 4, A B C D }
component_pack_impl! { 5, A B C D E }
component_pack_impl! { 6, A B C D E F }
component_pack_impl! { 7, A B C D E F G }
component_pack_impl! { 8, A B C D E F G H }
component_pack_impl! { 9, A B C D E F G H I }
component_pack_impl! { 10, A B C D E F G H I J }
component_pack_impl! { 11, A B C D E F G H I J K }
component_pack_impl! { 12, A B C D E F G H I J K L }
component_pack_impl! { 13, A B C D E F G H I J K L M }
component_pack_impl! { 14, A B C D E F G H I J K L M N }
component_pack_impl! { 15, A B C D E F G H I J K L M N O }
component_pack_impl! { 16, A B C D E F G H I J K L M N O P }
component_pack_impl! { 17, A B C D E F G H I J K L M N O P Q }
component_pack_impl! { 18, A B C D E F G H I J K L M N O P Q R }
component_pack_impl! { 19, A B C D E F G H I J K L M N O P Q R S }
component_pack_impl! { 20, A B C D E F G H I J K L M N O P Q R S T }
component_pack_impl! { 21, A B C D E F G H I J K L M N O P Q R S T U }
component_pack_impl! { 22, A B C D E F G H I J K L M N O P Q R S T U V }
component_pack_impl! { 23, A B C D E F G H I J K L M N O P Q R S T U V W }
component_pack_impl! { 24, A B C D E F G H I J K L M N O P Q R S T U V W X }
component_pack_impl! { 25, A B C D E F G H I J K L M N O P Q R S T U V W X Y }
component_pack_impl! { 26, A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
