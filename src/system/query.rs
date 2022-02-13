use crate::{archetype::access::DataBufferSet, entity::Entity};

pub struct QueryGenerator {}

/// A query is an iterator that holds the references to the components being accessed. Iteration
/// over a query must be VERY fast.
pub struct Query<D: DataBufferSet> {
    _remove_me: std::marker::PhantomData<D>,
}

impl QueryGenerator {
    pub fn new() -> Self {
        todo!()
    }

    /// Constructs a new query. Must ensure that the query being constructed is one that is allowed
    /// by what the system requested.
    pub fn create<D: DataBufferSet>(&self) -> Query<D> {
        todo!()
    }
}

impl<D: DataBufferSet> Query<D> {
    #[inline]
    fn is_empty(&self) -> bool {
        todo!()
    }

    #[inline]
    fn len(&self) -> usize {
        todo!()
    }
}

impl<D: DataBufferSet> Iterator for Query<D> {
    type Item = (Entity, D::Filter);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
