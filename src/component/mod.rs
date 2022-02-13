pub mod filter;

/// A component holds a type of data associated with an entity.
pub trait Component: Send + Sync {}
