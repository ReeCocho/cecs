use crate::component::filter::ComponentFilter;

pub struct QueryGenerator {}

/// A query is an iterator that holds the references to the components being accessed. Iteration
/// over a query must be VERY fast.
pub trait Query<C: ComponentFilter> {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

impl QueryGenerator {
    pub fn new() -> Self {
        todo!()
    }

    /// Constructs a new query. Must ensure that the query being constructed is one that is allowed
    /// by what the system requested.
    pub fn create<Q: Query<impl ComponentFilter>>(&self) -> Q {
        todo!()
    }
}
