use std::ptr::NonNull;

use crate::{
    archetype::{access::DataBufferSet, archetypes::Archetypes, Archetype},
    component::filter::ComponentFilter,
    entity::Entity,
    prw_lock::PrwReadHandle,
};

pub struct QueryGenerator<'a> {
    archetypes: &'a Archetypes,
    all_components: Archetype,
    mut_components: Archetype,
}

/// A query is an iterator that holds the references to the components being accessed. Iteration
/// over a query must be VERY fast.
pub struct Query<C: ComponentFilter> {
    /// List of storage sets and entity buffers to loop over.
    sets: Vec<(FastEntityIterator, C::StorageSet)>,
    /// Current working set and entity buffer.
    set: Option<(FastEntityIterator, C::StorageSet)>,
    /// Current working set index.
    idx: usize,
    len: usize,
}

/// Special fast iterator for entity storages.
struct FastEntityIterator {
    #[allow(dead_code)]
    handle: Option<PrwReadHandle<Vec<Entity>>>,
    ptr: NonNull<Entity>,
}

impl<'a> QueryGenerator<'a> {
    pub fn new<C: ComponentFilter>(archetypes: &'a Archetypes) -> Self {
        Self {
            archetypes,
            all_components: C::archetype(),
            mut_components: C::write_archetype(),
        }
    }

    /// Constructs a new query. Must ensure that the query being constructed is one that is allowed
    /// by what the system requested.
    pub fn create<C: ComponentFilter>(&self) -> Query<C> {
        assert!(C::read_archetype().subset_of(&self.all_components));
        assert!(C::write_archetype().subset_of(&self.mut_components));
        Query::new(self.archetypes)
    }
}

impl<C: ComponentFilter> Query<C> {
    fn new(archetypes: &Archetypes) -> Self {
        // Generate the archetype for the filter
        let archetype = C::archetype();

        let mut len = 0;

        // Find all archetype descriptors that our archetype is a subset of and generate data
        // buffer sets with them and their corresponding entites.
        let mut sets = Vec::default();
        for descriptor in archetypes.descriptors() {
            // Must be compatible
            if archetype.subset_of(&descriptor.archetype) {
                // Grab entity storage
                let handle = archetypes.get_entity_buffers().get(descriptor.entities);

                // Must have non-zero entity count
                if handle.len() != 0 {
                    len += handle.len();

                    // Add set and entity buffer
                    sets.push((
                        FastEntityIterator::new(handle),
                        C::make_storage_set(descriptor, archetypes),
                    ));
                }
            }
        }

        // Grab the starting set and entity buffer
        let set = sets.pop();

        Self {
            sets,
            set,
            idx: 0,
            len,
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

impl<C: ComponentFilter> Iterator for Query<C> {
    type Item = (Entity, <C::StorageSet as DataBufferSet>::Filter);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // Check if we have a working set
        if let Some((entities, set)) = &mut self.set {
            // Grab the filter and entity
            // NOTE: Safe to unwrap since sets are guaranteed not to be empty and if the set
            // wasn't valid last loop, it would have been replaced with a valid one.
            let filter = unsafe { set.fetch(self.idx) };
            let entity = unsafe { entities.fetch(self.idx) };

            // Move to the next set if the current is invalid
            self.idx += 1;
            if !set.is_valid(self.idx) {
                self.set = self.sets.pop();
                self.idx = 0;
            }

            Some((entity, filter))
        } else {
            None
        }
    }
}

impl Default for FastEntityIterator {
    #[inline]
    fn default() -> Self {
        Self {
            handle: None,
            ptr: NonNull::new(1 as *mut Entity).unwrap(),
        }
    }
}

impl FastEntityIterator {
    #[inline]
    fn new(handle: PrwReadHandle<Vec<Entity>>) -> Self {
        debug_assert!(handle.len() != 0);

        // Cast const to mut, but we never modify the buffer so it's totally cool
        let ptr = handle.as_ptr() as *mut Entity;
        FastEntityIterator {
            handle: Some(handle),
            // Safe to unwrap since len != 0, which means the buffer must be allocated
            ptr: NonNull::new(ptr).expect("Empty lock given to fast entity iterator"),
        }
    }

    #[inline(always)]
    unsafe fn fetch(&mut self, idx: usize) -> Entity {
        *self.ptr.as_ptr().add(idx)
    }
}
