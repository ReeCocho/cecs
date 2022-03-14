pub mod archetype;
pub mod component;
pub mod dispatcher;
pub mod entity;
pub mod prw_lock;
pub mod system;
pub mod world;

#[cfg(test)]
mod tests {
    use crate::archetype::Archetype;
    use crate::{dispatcher::Dispatcher, system::System, world::World};
    use std::any::TypeId;

    struct TestSystem;

    /*
    impl System for TestSystem {
        type Components = ();

        fn tick(&mut self, gen: crate::system::query::QueryGenerator) {
            println!("Ayyy, I'm running here");
        }
    }
    */

    #[test]
    fn basic_example() {
        // Create the world
        let mut world = World::new();

        // TODO: Add a way to create entities with components attached

        // Create the dispatcher
        let mut dispatcher = Dispatcher::builder().build();

        // Run the dispatcher
        dispatcher.run(&mut world);
    }

    #[test]
    fn check_set_comparisons() {
        let mut one = Archetype::default();
        let mut two = Archetype::default();
        one.add_component_by_id(TypeId::of::<u32>());
        two.add_component_by_id(TypeId::of::<u32>());
        assert_eq!(one.subset_of(&two), true);

        one.add_component_by_id(TypeId::of::<i64>());
        assert_eq!(one.subset_of(&two), false);
        assert_eq!(two.subset_of(&one), true);

        assert_eq!(1, 1);
    }
}
