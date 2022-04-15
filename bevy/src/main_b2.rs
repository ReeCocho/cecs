use std::time::{Duration, Instant};

use bevy_ecs::prelude::*;
extern crate num_cpus;

use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
};

//10_000_000
fn main() {
    let iteration_var: Vec<usize> = vec![1_000, 100_000, 1_000_000];

    println!("Bevy vs CECS benchmark");
    for i in &iteration_var
    {
        compare_bench(*i);
    }
    println!();
    println!("Thread runtime benchmark");
    for i in &iteration_var
    {
        thread_bench(*i);
        println!();
    }

}

fn thread_bench(num_iterations: usize)
{
    let mut i = 1;
    println!("Time To Run with {} iterations:", num_iterations);
    while i <= 8
    {
        let time = cecs_bench(i, num_iterations);
        println!("{} threads, {} ms", i, time.as_millis());
        i *= 2;
    }
}

fn compare_bench(num_iterations: usize)
{
    let t_count = num_cpus::get();
    let cecs = cecs_bench(t_count, num_iterations);
    let bevy = bevy_bench( t_count, num_iterations);
    let c2 = cecs_bench(t_count, num_iterations);
    let b2 = bevy_bench(t_count, num_iterations);
    let c3 = cecs_bench(t_count, num_iterations);
    let b3 = bevy_bench(t_count, num_iterations);

    println!("\nIterations:  {}", num_iterations);
    println!("cecs : {}ms, {}ms, {}ms", cecs.as_millis(), c2.as_millis(), c3.as_millis());
    println!("bevy : {}ms, {}ms, {}ms", bevy.as_millis(), b2.as_millis(), b3.as_millis());
}

fn cecs_bench(t_count: usize, num_iterations: usize) -> Duration {
    let mut world = cecs::world::World::new();

    world.create((
        vec![Component1(0)], vec![Component2(0)], vec![Component3(0)]
    ));
    world.create((
        vec![Component3(0)], vec![Component4(0)], vec![Component5(0)]
    ));
    world.create((
        vec![Component2(0)], vec![Component3(0)], vec![Component4(0)]
    ));
    world.create((
        vec![Component5(0)],
    ));
    world.create((
        vec![Component1(0)], vec![Component3(0)], vec![Component5(0)]
    ));
    

    // Create the dispatcher
    let mut dispatcher = Dispatcher::builder().thread_count( t_count );
    dispatcher.with_system(System0, &[]);
    dispatcher.with_system(System1, &[]);
    dispatcher.with_system(System2, &[]);
    dispatcher.with_system(System3, &[]);
    dispatcher.with_system(System4, &[]);
    dispatcher.with_system(System5, &[]);

    

    let mut dispatcher = dispatcher.build();

    let start = Instant::now();

    for _ in 0..num_iterations {
        dispatcher.run(&mut world);
    }

    Instant::now().duration_since(start)
}

fn bevy_bench(t_count: usize, num_iterations: usize) -> Duration {
    let mut world = World::new();
    world.insert_resource(ComputeTaskPool(TaskPoolBuilder::new().num_threads(t_count).build()));

    // Static //
    world.spawn()
        .insert(Component1(0))
        .insert(Component2(0))
        .insert(Component3(0));

    world.spawn()
        .insert(Component3(0))
        .insert(Component4(0))
        .insert(Component5(0));

    world.spawn()
        .insert(Component2(0))
        .insert(Component3(0))
        .insert(Component4(0));
    
    world.spawn()
        .insert(Component5(0));

    world.spawn()
        .insert(Component1(0))
        .insert(Component3(0))
        .insert(Component5(0));
    

    let mut schedule = Schedule::default();
    
    //*  ADD HERE *//
    schedule.add_stage("base_stage", SystemStage::parallel()
	.with_system(System0::bevy_system)
	.with_system(System1::bevy_system)
	.with_system(System2::bevy_system)
	.with_system(System3::bevy_system)
	.with_system(System4::bevy_system)
	.with_system(System5::bevy_system)
);

    //*______________ *//

    let start = Instant::now();

    for _ in 0..num_iterations {
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

new_component! { Component0 }
new_component! { Component1 }
new_component! { Component2 }
new_component! { Component3 }
new_component! { Component4 }
new_component! { Component5 }
new_component! { Component6 }
new_component! { Component7 }
new_component! { Component8 }


new_system! { System0, Component0, Component1 Component2 Component3 Component4 Component5 }
new_system! { System1, Component1, Component0 Component2 Component3 Component4 Component5 }
new_system! { System2, Component2, Component0 Component1 Component3 Component4 Component5 }
new_system! { System3, Component3, Component0 Component1 Component2 Component4 Component5 }
new_system! { System4, Component4, Component0 Component1 Component2 Component3 Component5 }
new_system! { System5, Component5, Component0 Component1 Component2 Component3 Component4 }
