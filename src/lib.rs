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
    use crate::component::filter::{Read, Write};
    use crate::component::Component;
    use crate::{dispatcher::Dispatcher, system::System, world::World};
    use std::any::TypeId;

    struct ComponentA(u32);
    struct ComponentB(u32);
    struct ComponentC(u32);

    impl Component for ComponentA {}
    impl Component for ComponentB {}
    impl Component for ComponentC {}

    struct SystemA;

    impl System for SystemA {
        type Components = (Read<ComponentA>, Write<ComponentB>);

        fn tick(&mut self, gen: crate::system::query::QueryGenerator) {
            for (i, (_, (a, b))) in gen
                .create::<(Read<ComponentA>, Read<ComponentB>)>()
                .into_iter()
                .enumerate()
            {
                assert!(a.0 == i as u32 + 1);
                assert!(b.0 == i as u32 + 4);
            }

            for (i, (_, (a, b))) in gen
                .create::<(Read<ComponentA>, Write<ComponentB>)>()
                .into_iter()
                .enumerate()
            {
                assert!(a.0 == i as u32 + 1);
                assert!(b.0 == i as u32 + 4);
            }
        }
    }

    struct SystemB;

    impl System for SystemB {
        type Components = (Write<ComponentB>, Read<ComponentC>);

        fn tick(&mut self, gen: crate::system::query::QueryGenerator) {
            for (i, (_, (b, c))) in gen
                .create::<(Read<ComponentB>, Read<ComponentC>)>()
                .into_iter()
                .enumerate()
            {
                assert!(b.0 == i as u32 + 1);
                assert!(c.0 == i as u32 + 4);
            }

            for (i, (_, (b, c))) in gen
                .create::<(Write<ComponentB>, Read<ComponentC>)>()
                .into_iter()
                .enumerate()
            {
                assert!(b.0 == i as u32 + 1);
                assert!(c.0 == i as u32 + 4);
            }
        }
    }

    #[test]
    fn basic_example() {
        // Create the world
        let mut world = World::new();

        // Create entities
        world.create((
            vec![ComponentA(1), ComponentA(2), ComponentA(3)],
            vec![ComponentB(4), ComponentB(5), ComponentB(6)],
        ));

        world.create((
            vec![ComponentB(1), ComponentB(2), ComponentB(3)],
            vec![ComponentC(4), ComponentC(5), ComponentC(6)],
        ));

        // Create the dispatcher
        let mut dispatcher = Dispatcher::builder()
            .with_system(SystemA)
            .with_system(SystemB)
            .build();

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
