pub mod archetype;
pub mod component;
pub mod dispatcher;
pub mod entity;
pub mod prw_lock;
pub mod system;
pub mod world;

#[cfg(test)]
mod tests {
    use crate::{dispatcher::Dispatcher, system::System, world::World};

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
}
