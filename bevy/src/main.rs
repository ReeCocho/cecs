use std::{time::{Duration, Instant}, sync::Arc};

use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};

use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
};
use specs::rayon::{ThreadPool, ThreadPoolBuilder};

const ITER_COUNT: usize = 500_000;
const THREAD_COUNT: usize = 4;

fn main() {
    let cecs = cecs_bench();
    let bevy = bevy_bench();
    let specs = specs_bench();

    println!("cecs : {} ms", cecs.as_millis());
    println!("bevy : {} ms", bevy.as_millis());
    println!("specs : {} ms", specs.as_millis());
}

fn cecs_bench() -> Duration {
    let mut world = cecs::world::World::new();

    // Create the dispatcher
    let mut dispatcher = Dispatcher::builder().thread_count(THREAD_COUNT);
    dispatcher.with_system(S11, &[]);
    dispatcher.with_system(S21, &[]);
    dispatcher.with_system(S31, &[]);
    dispatcher.with_system(S41, &[]);
    dispatcher.with_system(S51, &[]);
    dispatcher.with_system(S22, &[]);
    dispatcher.with_system(S32, &[]);
    dispatcher.with_system(S42, &[]);
    dispatcher.with_system(S52, &[]);
    dispatcher.with_system(S33, &[]);
    dispatcher.with_system(S43, &[]);
    dispatcher.with_system(S53, &[]);
    dispatcher.with_system(S44, &[]);
    dispatcher.with_system(S54, &[]);
    dispatcher.with_system(S55, &[]);

    let mut dispatcher = dispatcher.build();

    let start = Instant::now();

    for _ in 0..ITER_COUNT {
        dispatcher.run(&mut world);
    }

    Instant::now().duration_since(start)
}

fn bevy_bench() -> Duration {
    use bevy_ecs::prelude::*;

    let mut world = World::new();
    world.insert_resource(ComputeTaskPool(TaskPoolBuilder::new().num_threads(THREAD_COUNT).build()));

    let mut schedule = Schedule::default();
    schedule.add_stage(
        "base_stage",
        SystemStage::parallel()
            .with_system(S11::bevy_system)
            .with_system(S21::bevy_system)
            .with_system(S31::bevy_system)
            .with_system(S41::bevy_system)
            .with_system(S51::bevy_system)
            .with_system(S22::bevy_system)
            .with_system(S32::bevy_system)
            .with_system(S42::bevy_system)
            .with_system(S52::bevy_system)
            .with_system(S33::bevy_system)
            .with_system(S43::bevy_system)
            .with_system(S53::bevy_system)
            .with_system(S44::bevy_system)
            .with_system(S54::bevy_system)
            .with_system(S55::bevy_system)
    );

    let start = Instant::now();

    for _ in 0..ITER_COUNT {
        schedule.run(&mut world);
    }

    Instant::now().duration_since(start)
}

fn specs_bench() -> Duration {
    use specs::prelude::*;

    let mut world = World::new();
    world.register::<C0>(); 
    world.register::<C1>(); 
    world.register::<C2>(); 
    world.register::<C3>(); 
    world.register::<C4>(); 
    world.register::<C5>(); 
    world.register::<C6>(); 
    world.register::<C7>(); 
    world.register::<C8>(); 
    world.register::<C9>(); 
    world.register::<C10>();
    world.register::<C11>();
    world.register::<C12>();
    world.register::<C13>();
    world.register::<C14>();
    world.register::<C15>();
    world.register::<C16>();
    world.register::<C17>();
    world.register::<C18>();
    world.register::<C19>();
    world.register::<C20>();
    world.register::<C21>();
    world.register::<C22>();
    world.register::<C23>();
    world.register::<C24>();
    world.register::<C25>();

    let mut dispatcher = DispatcherBuilder::new()
    .with(S11, "sys_1", &[])
    .with(S21, "sys_2", &[])
    .with(S22, "sys_3", &[])
    .with(S31, "sys_4", &[])
    .with(S32, "sys_5", &[])
    .with(S33, "sys_6", &[])
    .with(S41, "sys_7", &[])
    .with(S42, "sys_8", &[])
    .with(S43, "sys_9", &[])
    .with(S44, "sys_10", &[])
    .with(S51, "sys_11", &[])
    .with(S52, "sys_12", &[])
    .with(S53, "sys_13", &[])
    .with(S54, "sys_14", &[])
    .with(S55, "sys_15", &[])
    .with_pool(Arc::new(ThreadPoolBuilder::new().num_threads(THREAD_COUNT).build().unwrap()))
    .build();

    dispatcher.setup(&mut world);

    let start = Instant::now();

    for _ in 0..ITER_COUNT {
        dispatcher.dispatch(&mut world);
    }

    Instant::now().duration_since(start)
}

/// Helper macro to create components
macro_rules! new_component {
    ($name:ident) => {
        #[derive(bevy_ecs::prelude::Component, Copy, Clone)]
        struct $name(u32);

        impl specs::prelude::Component for $name {
            type Storage = specs::prelude::VecStorage<Self>;
        }

        impl Component for $name {}
    };
}

/// Helper macro to create systems
macro_rules! new_system {
    ($name:ident, $( $comp_w:ident )*, $( $comp_r:ident )* ) => {
        struct $name;

        impl $name {
            fn bevy_system(_: bevy_ecs::prelude::Query<( $(&mut $comp_w,)* $(&$comp_r,)* )>) {
                
            }
        }

        impl<'a> specs::prelude::System<'a> for $name {
            type SystemData = ( $(specs::prelude::WriteStorage<'a, $comp_w>,)* $(specs::prelude::ReadStorage<'a, $comp_r>,)* );

            fn run(&mut self, _: Self::SystemData) {

            }
        }

        impl System for $name {
            type Components = ( $(Write<$comp_w>,)* $(Read<$comp_r>,)* );

            fn tick(&mut self, _: QueryGenerator) {
                
            }
        }
    };
}

new_component! { C0 }
new_component! { C1 }
new_component! { C2 }
new_component! { C3 }
new_component! { C4 }
new_component! { C5 }
new_component! { C6 }
new_component! { C7 }
new_component! { C8 }
new_component! { C9 }
new_component! { C10 }
new_component! { C11 }
new_component! { C12 }
new_component! { C13 }
new_component! { C14 }
new_component! { C15 }
new_component! { C16 }
new_component! { C17 }
new_component! { C18 }
new_component! { C19 }
new_component! { C20 }
new_component! { C21 }
new_component! { C22 }
new_component! { C23 }
new_component! { C24 }
new_component! { C25 }

new_system! { S11, C1, C2 C3 C4 C5 C6 C7 C8 C9 C10 C11 C12 C13 C14 C15 }

new_system! { S21, C2, C1 C4 C5 C6 C7 C8 C9 C10 C11 C12 C13 C14 C15 }
new_system! { S22, C3, C1 C4 C5 C6 C7 C8 C9 C10 C11 C12 C13 C14 C15 }

new_system! { S31, C4, C1 C2 C3 C7 C8 C9 C10 C11 C12 C13 C14 C15 }
new_system! { S32, C5, C1 C2 C3 C7 C8 C9 C10 C11 C12 C13 C14 C15 }
new_system! { S33, C6, C1 C2 C3 C7 C8 C9 C10 C11 C12 C13 C14 C15 }

new_system! { S41, C7,  C1 C2 C3 C4 C5 C6 C11 C12 C13 C14 C15 }
new_system! { S42, C8,  C1 C2 C3 C4 C5 C6 C11 C12 C13 C14 C15}
new_system! { S43, C9,  C1 C2 C3 C4 C5 C6 C11 C12 C13 C14 C15 }
new_system! { S44, C10, C1 C2 C3 C4 C5 C6 C11 C12 C13 C14 C15 }

new_system! { S51, C11, C1 C2 C3 C4 C5 C6 C7 C8 C9 C10 }
new_system! { S52, C12, C1 C2 C3 C4 C5 C6 C7 C8 C9 C10 }
new_system! { S53, C13, C1 C2 C3 C4 C5 C6 C7 C8 C9 C10 }
new_system! { S54, C14, C1 C2 C3 C4 C5 C6 C7 C8 C9 C10 }
new_system! { S55, C15, C1 C2 C3 C4 C5 C6 C7 C8 C9 C10 }