use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
    world::World,
};

struct ComponentA(u32);
struct ComponentB(u32);
struct ComponentC(u32);

impl Component for ComponentA {}
impl Component for ComponentB {}
impl Component for ComponentC {}

struct SystemA;

impl System for SystemA {
    type Components = (Read<ComponentA>, Write<ComponentB>);

    fn tick(&mut self, gen: QueryGenerator) {
        let mut count = 0;

        for (i, (_, (a, b))) in gen
            .create::<(Read<ComponentA>, Read<ComponentB>)>()
            .into_iter()
            .enumerate()
        {
            assert!(a.0 == i as u32 + 1);
            assert!(b.0 == i as u32 + 4);
            count += 1;
        }

        for (i, (_, (a, b))) in gen
            .create::<(Read<ComponentA>, Write<ComponentB>)>()
            .into_iter()
            .enumerate()
        {
            assert!(a.0 == i as u32 + 1);
            assert!(b.0 == i as u32 + 4);
            count += 1;
        }

        assert_eq!(count, 6);
    }
}

struct SystemB;

impl System for SystemB {
    type Components = (Write<ComponentB>, Read<ComponentC>);

    fn tick(&mut self, gen: QueryGenerator) {
        let mut count = 0;

        for (i, (_, (b, c))) in gen
            .create::<(Read<ComponentB>, Read<ComponentC>)>()
            .into_iter()
            .enumerate()
        {
            assert!(b.0 == i as u32 + 1);
            assert!(c.0 == i as u32 + 4);
            count += 1;
        }

        for (i, (_, (b, c))) in gen
            .create::<(Write<ComponentB>, Read<ComponentC>)>()
            .into_iter()
            .enumerate()
        {
            assert!(b.0 == i as u32 + 1);
            assert!(c.0 == i as u32 + 4);
            count += 1;
        }

        assert_eq!(count, 6);
    }
}

fn main() {
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
    let mut dispatcher = Dispatcher::builder().thread_count(4);
    dispatcher.with_system(SystemA, &[]);
    dispatcher.with_system(SystemB, &[]);

    let mut dispatcher = dispatcher.build();

    // Run the dispatcher
    for _ in 0..10_000 {
        dispatcher.run(&mut world);
    }
}
