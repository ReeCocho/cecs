pub mod query;

use crate::component::filter::ComponentFilter;

use self::query::QueryGenerator;

/// A system is what performs the actual logic within an ECS. It operates on a subset of entities
/// that match a particular archetype.
pub trait System {
    /// When creating a system, you use this type to define what subset of components your
    /// system is going to operate on.
    type Components: ComponentFilter;

    /// Runs a single iteration of the system.
    fn tick(&mut self, gen: QueryGenerator);
}

pub trait GenericSystem {
    fn generic_tick(&mut self, gen: QueryGenerator);
}

impl<T: System> GenericSystem for T {
    fn generic_tick(&mut self, gen: QueryGenerator) {
        self.tick(gen);
    }
}
