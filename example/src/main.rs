use cecs::{
    component::{
        filter::{Read, Write},
        Component,
    },
    dispatcher::Dispatcher,
    system::{query::QueryGenerator, System},
    world::World,
};

fn main() {
    // Create the world
    let mut world = World::new();

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

    // Run the dispatcher
    for _ in 0..10_000 {
        dispatcher.run(&mut world);
    }
}

/// Helper macro to create components
macro_rules! new_component {
    ($name:ident) => {
        struct $name(u32);
        impl Component for $name {}
    };
}

/// Helper macro to create systems
macro_rules! new_system {
    ($name:ident, $( $comp_w:ident )*, $( $comp_r:ident )* ) => {
        struct $name;

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
new_system! { SystemG, ComponentG, ComponentA ComponentB ComponentD ComponentE ComponentG ComponentH ComponentI }
new_system! { SystemH, ComponentH, ComponentA ComponentB ComponentC ComponentE ComponentF ComponentG ComponentI }
new_system! { SystemI, ComponentI, ComponentA ComponentB ComponentC ComponentG ComponentH }
