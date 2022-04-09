use std::time::{Duration, Instant};

use bevy_ecs::prelude::*;

use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
};

const ITER_COUNT: usize = 10_000_000;

fn main() {
    let cecs = cecs_bench();
    let bevy = bevy_bench();

    println!("cecs : {} ms", cecs.as_millis());
    println!("bevy : {} ms", bevy.as_millis());
}

fn cecs_bench() -> Duration {
    let mut world = cecs::world::World::new();

    world.create((vec![ComponentA(0)], vec![ComponentC(0)], vec![ComponentD(0)], vec![ComponentF(0)], vec![ComponentG(0)], vec![ComponentH(0)], vec![ComponentI(0)]));
    world.create((vec![ComponentB(0)], vec![ComponentD(0)], vec![ComponentG(0)], vec![ComponentH(0)], vec![ComponentI(0)]));
    world.create((vec![ComponentC(0)], vec![ComponentA(0)], vec![ComponentD(0)], vec![ComponentH(0)], vec![ComponentI(0)]));
    world.create((vec![ComponentD(0)], vec![ComponentA(0)], vec![ComponentC(0)], vec![ComponentF(0)], vec![ComponentG(0)]));
    world.create((vec![ComponentE(0)], vec![ComponentG(0)], vec![ComponentH(0)]));
    world.create((vec![ComponentF(0)], vec![ComponentA(0)], vec![ComponentD(0)], vec![ComponentH(0)]));
    world.create((vec![ComponentG(0)], vec![ComponentA(0)], vec![ComponentB(0)], vec![ComponentD(0)], vec![ComponentE(0)], vec![ComponentH(0)], vec![ComponentI(0)]));
    world.create((vec![ComponentH(0)], vec![ComponentA(0)], vec![ComponentB(0)], vec![ComponentC(0)], vec![ComponentE(0)], vec![ComponentF(0)], vec![ComponentG(0)], vec![ComponentI(0)]));
    world.create((vec![ComponentI(0)], vec![ComponentA(0)], vec![ComponentB(0)], vec![ComponentC(0)], vec![ComponentG(0)], vec![ComponentH(0)]));

    // Create the dispatcher
    let mut dispatcher = Dispatcher::builder().thread_count(12);
    dispatcher.with_system(SystemA, &[]);
    dispatcher.with_system(SystemB, &[]);
    dispatcher.with_system(SystemC, &[]);
    dispatcher.with_system(SystemD, &[]);
    dispatcher.with_system(SystemE, &[]);
    dispatcher.with_system(SystemF, &[]);
    dispatcher.with_system(SystemG, &[]);
    dispatcher.with_system(SystemH, &[]);
    dispatcher.with_system(SystemI, &[]);

    let mut dispatcher = dispatcher.build();

    let start = Instant::now();

    for _ in 0..ITER_COUNT {
        dispatcher.run(&mut world);
    }

    Instant::now().duration_since(start)
}

fn bevy_bench() -> Duration {
    let mut world = World::new();

    world
        .spawn()
        .insert(ComponentA(0))
        .insert(ComponentC(0))
        .insert(ComponentD(0))
        .insert(ComponentF(0))
        .insert(ComponentG(0))
        .insert(ComponentH(0))
        .insert(ComponentI(0));

    world
        .spawn()
        .insert(ComponentB(0))
        .insert(ComponentD(0))
        .insert(ComponentG(0))
        .insert(ComponentH(0))
        .insert(ComponentI(0));

    world
        .spawn()
        .insert(ComponentC(0))
        .insert(ComponentA(0))
        .insert(ComponentD(0))
        .insert(ComponentH(0))
        .insert(ComponentI(0));

    world
        .spawn()
        .insert(ComponentD(0))
        .insert(ComponentA(0))
        .insert(ComponentC(0))
        .insert(ComponentF(0))
        .insert(ComponentG(0));

    world
        .spawn()
        .insert(ComponentE(0))
        .insert(ComponentG(0))
        .insert(ComponentH(0));

    world
        .spawn()
        .insert(ComponentF(0))
        .insert(ComponentA(0))
        .insert(ComponentD(0))
        .insert(ComponentH(0));

    world
        .spawn()
        .insert(ComponentG(0))
        .insert(ComponentA(0))
        .insert(ComponentB(0))
        .insert(ComponentD(0))
        .insert(ComponentE(0))
        .insert(ComponentH(0))
        .insert(ComponentI(0));

    world
        .spawn()
        .insert(ComponentH(0))
        .insert(ComponentA(0))
        .insert(ComponentB(0))
        .insert(ComponentC(0))
        .insert(ComponentE(0))
        .insert(ComponentF(0))
        .insert(ComponentG(0))
        .insert(ComponentI(0));

    world
        .spawn()
        .insert(ComponentI(0))
        .insert(ComponentA(0))
        .insert(ComponentB(0))
        .insert(ComponentC(0))
        .insert(ComponentG(0))
        .insert(ComponentH(0));

    let mut schedule = Schedule::default();
    schedule.add_stage(
        "base_stage",
        SystemStage::parallel()
            .with_system(SystemA::bevy_system)
            .with_system(SystemB::bevy_system)
            .with_system(SystemC::bevy_system)
            .with_system(SystemD::bevy_system)
            .with_system(SystemE::bevy_system)
            .with_system(SystemF::bevy_system)
            .with_system(SystemG::bevy_system)
            .with_system(SystemH::bevy_system)
            .with_system(SystemI::bevy_system),
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
        #[derive(Component)]
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

new_component! { ComponentA }
new_component! { ComponentB }
new_component! { ComponentC }
new_component! { ComponentD }
new_component! { ComponentE }
new_component! { ComponentF }
new_component! { ComponentG }
new_component! { ComponentH }
new_component! { ComponentI }

new_system! { SystemA, ComponentA, ComponentC ComponentD ComponentF ComponentG ComponentH ComponentI }
new_system! { SystemB, ComponentB, ComponentD ComponentG ComponentH ComponentI }
new_system! { SystemC, ComponentC, ComponentA ComponentD ComponentH ComponentI }
new_system! { SystemD, ComponentD, ComponentA ComponentC ComponentF ComponentG }
new_system! { SystemE, ComponentE, ComponentG ComponentH }
new_system! { SystemF, ComponentF, ComponentA ComponentD ComponentH }
new_system! { SystemG, ComponentG, ComponentA ComponentB ComponentD ComponentE ComponentH ComponentI }
new_system! { SystemH, ComponentH, ComponentA ComponentB ComponentC ComponentE ComponentF ComponentG ComponentI }
new_system! { SystemI, ComponentI, ComponentA ComponentB ComponentC ComponentG ComponentH }
