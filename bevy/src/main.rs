use std::time::{Duration, Instant};

use bevy_ecs::prelude::*;

use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
};

const ITER_COUNT: usize = 500_000;

fn main() {
    let cecs = cecs_bench();
    let bevy = bevy_bench();

    println!("cecs : {} ms", cecs.as_millis());
    println!("bevy : {} ms", bevy.as_millis());
}

fn cecs_bench() -> Duration {
    let mut world = cecs::world::World::new();

    // Create the dispatcher
    let mut dispatcher = Dispatcher::builder().thread_count(4);
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
    let mut world = World::new();
    world.insert_resource(ComputeTaskPool(TaskPoolBuilder::new().num_threads(4).build()));

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

/// Helper macro to create components
macro_rules! new_component {
    ($name:ident) => {
        #[derive(Component, Copy, Clone)]
        struct $name(u32);

        impl Component for $name {}
    };
}

/// Helper macro to create systems
macro_rules! new_system {
    ($name:ident, $( $comp_w:ident )*, $( $comp_r:ident )* ) => {
        struct $name;

        impl $name {
            fn bevy_system(_: Query<( $(&mut $comp_w,)* $(&$comp_r,)* )>) {
                
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