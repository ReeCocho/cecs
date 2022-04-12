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
    dispatcher.with_system(S22, &[]);
    dispatcher.with_system(S31, &[]);
    dispatcher.with_system(S32, &[]);
    dispatcher.with_system(S33, &[]);
    dispatcher.with_system(S41, &[]);
    dispatcher.with_system(S42, &[]);
    dispatcher.with_system(S43, &[]);
    dispatcher.with_system(S44, &[]);
    dispatcher.with_system(S51, &[]);
    dispatcher.with_system(S52, &[]);
    dispatcher.with_system(S53, &[]);
    dispatcher.with_system(S54, &[]);
    dispatcher.with_system(S55, &[]);
    dispatcher.with_system(S61, &[]);
    dispatcher.with_system(S62, &[]);
    dispatcher.with_system(S63, &[]);
    dispatcher.with_system(S64, &[]);
    dispatcher.with_system(S65, &[]);
    dispatcher.with_system(S66, &[]);
    dispatcher.with_system(S71, &[]);
    dispatcher.with_system(S72, &[]);
    dispatcher.with_system(S73, &[]);
    dispatcher.with_system(S74, &[]);
    dispatcher.with_system(S75, &[]);
    dispatcher.with_system(S76, &[]);
    dispatcher.with_system(S77, &[]);
    dispatcher.with_system(S81, &[]);
    dispatcher.with_system(S82, &[]);
    dispatcher.with_system(S83, &[]);
    dispatcher.with_system(S84, &[]);
    dispatcher.with_system(S85, &[]);
    dispatcher.with_system(S86, &[]);
    dispatcher.with_system(S87, &[]);
    dispatcher.with_system(S88, &[]);
    dispatcher.with_system(S91, &[]);
    dispatcher.with_system(S92, &[]);
    dispatcher.with_system(S93, &[]);
    dispatcher.with_system(S94, &[]);
    dispatcher.with_system(S95, &[]);
    dispatcher.with_system(S96, &[]);
    dispatcher.with_system(S97, &[]);
    dispatcher.with_system(S98, &[]);
    dispatcher.with_system(S99, &[]);
    dispatcher.with_system(S101, &[]);
    dispatcher.with_system(S102, &[]);
    dispatcher.with_system(S103, &[]);
    dispatcher.with_system(S104, &[]);
    dispatcher.with_system(S105, &[]);
    dispatcher.with_system(S106, &[]);
    dispatcher.with_system(S107, &[]);
    dispatcher.with_system(S108, &[]);
    dispatcher.with_system(S109, &[]);
    dispatcher.with_system(S1010, &[]);
    dispatcher.with_system(S111, &[]);
    dispatcher.with_system(S112, &[]);
    dispatcher.with_system(S113, &[]);
    dispatcher.with_system(S114, &[]);
    dispatcher.with_system(S115, &[]);
    dispatcher.with_system(S116, &[]);
    dispatcher.with_system(S117, &[]);
    dispatcher.with_system(S118, &[]);
    dispatcher.with_system(S119, &[]);
    dispatcher.with_system(S1110, &[]);
    dispatcher.with_system(S1111, &[]);
    dispatcher.with_system(S121, &[]);
    dispatcher.with_system(S122, &[]);
    dispatcher.with_system(S123, &[]);
    dispatcher.with_system(S124, &[]);
    dispatcher.with_system(S125, &[]);
    dispatcher.with_system(S126, &[]);
    dispatcher.with_system(S127, &[]);
    dispatcher.with_system(S128, &[]);
    dispatcher.with_system(S129, &[]);
    dispatcher.with_system(S1210, &[]);
    dispatcher.with_system(S1211, &[]);
    dispatcher.with_system(S1212, &[]);

    let mut dispatcher = dispatcher.build();

    let start = Instant::now();

    for _ in 0..ITER_COUNT {
        dispatcher.run(&mut world);
    }

    Instant::now().duration_since(start)
}

fn bevy_bench() -> Duration {
    let mut world = World::new();

    let mut schedule = Schedule::default();
    schedule.add_stage(
        "base_stage",
        SystemStage::parallel()
            .with_system(S11::bevy_system)
            .with_system(S21::bevy_system)
            .with_system(S22::bevy_system)
            .with_system(S31::bevy_system)
            .with_system(S32::bevy_system)
            .with_system(S33::bevy_system)
            .with_system(S41::bevy_system)
            .with_system(S42::bevy_system)
            .with_system(S43::bevy_system)
            .with_system(S44::bevy_system)
            .with_system(S51::bevy_system)
            .with_system(S52::bevy_system)
            .with_system(S53::bevy_system)
            .with_system(S54::bevy_system)
            .with_system(S55::bevy_system)
            .with_system(S61::bevy_system)
            .with_system(S62::bevy_system)
            .with_system(S63::bevy_system)
            .with_system(S64::bevy_system)
            .with_system(S65::bevy_system)
            .with_system(S66::bevy_system)
            .with_system(S71::bevy_system)
            .with_system(S72::bevy_system)
            .with_system(S73::bevy_system)
            .with_system(S74::bevy_system)
            .with_system(S75::bevy_system)
            .with_system(S76::bevy_system)
            .with_system(S77::bevy_system)
            .with_system(S81::bevy_system)
            .with_system(S82::bevy_system)
            .with_system(S83::bevy_system)
            .with_system(S84::bevy_system)
            .with_system(S85::bevy_system)
            .with_system(S86::bevy_system)
            .with_system(S87::bevy_system)
            .with_system(S88::bevy_system)
            .with_system(S91::bevy_system)
            .with_system(S92::bevy_system)
            .with_system(S93::bevy_system)
            .with_system(S94::bevy_system)
            .with_system(S95::bevy_system)
            .with_system(S96::bevy_system)
            .with_system(S97::bevy_system)
            .with_system(S98::bevy_system)
            .with_system(S99::bevy_system)
            .with_system(S101::bevy_system)
            .with_system(S102::bevy_system)
            .with_system(S103::bevy_system)
            .with_system(S104::bevy_system)
            .with_system(S105::bevy_system)
            .with_system(S106::bevy_system)
            .with_system(S107::bevy_system)
            .with_system(S108::bevy_system)
            .with_system(S109::bevy_system)
            .with_system(S1010::bevy_system)
            .with_system(S111::bevy_system)
            .with_system(S112::bevy_system)
            .with_system(S113::bevy_system)
            .with_system(S114::bevy_system)
            .with_system(S115::bevy_system)
            .with_system(S116::bevy_system)
            .with_system(S117::bevy_system)
            .with_system(S118::bevy_system)
            .with_system(S119::bevy_system)
            .with_system(S1110::bevy_system)
            .with_system(S1111::bevy_system)
            .with_system(S121::bevy_system)
            .with_system(S122::bevy_system)
            .with_system(S123::bevy_system)
            .with_system(S124::bevy_system)
            .with_system(S125::bevy_system)
            .with_system(S126::bevy_system)
            .with_system(S127::bevy_system)
            .with_system(S128::bevy_system)
            .with_system(S129::bevy_system)
            .with_system(S1210::bevy_system)
            .with_system(S1211::bevy_system)
            .with_system(S1212::bevy_system),
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

new_system! { S11, C2 C3 C4 C5 C6 C7 C8 C9 C10 C11 C12, C1 }

new_system! { S21, C1 C3 C4 C5 C6 C7 C8 C9 C10 C11 C12, C2 }
new_system! { S22, , C2 }

new_system! { S31, C2 C1 C4 C5 C6 C7 C8 C9 C10 C11 C12, C3 }
new_system! { S32, , C3 }
new_system! { S33, , C3 }

new_system! { S41, C2 C3 C1 C5 C6 C7 C8 C9 C10 C11 C12, C4 }
new_system! { S42, , C4 }
new_system! { S43, , C4 }
new_system! { S44, , C4 }

new_system! { S51, C2 C3 C4 C1 C6 C7 C8 C9 C10 C11 C12, C5 }
new_system! { S52, , C5 }
new_system! { S53, , C5 }
new_system! { S54, , C5 }
new_system! { S55, , C5 }

new_system! { S61, C2 C3 C4 C5 C1 C7 C8 C9 C10 C11 C12, C6 }
new_system! { S62, , C6 }
new_system! { S63, , C6 }
new_system! { S64, , C6 }
new_system! { S65, , C6 }
new_system! { S66, , C6 }

new_system! { S71, C2 C3 C4 C5 C6 C1 C8 C9 C10 C11 C12, C7 }
new_system! { S72, , C7 }
new_system! { S73, , C7 }
new_system! { S74, , C7 }
new_system! { S75, , C7 }
new_system! { S76, , C7 }
new_system! { S77, , C7 }

new_system! { S81, C2 C3 C4 C5 C6 C7 C1 C9 C10 C11 C12, C8 }
new_system! { S82, , C8 }
new_system! { S83, , C8 }
new_system! { S84, , C8 }
new_system! { S85, , C8 }
new_system! { S86, , C8 }
new_system! { S87, , C8 }
new_system! { S88, , C8 }

new_system! { S91, C2 C3 C4 C5 C6 C7 C8 C1 C10 C11 C12, C9 }
new_system! { S92, , C9 }
new_system! { S93, , C9 }
new_system! { S94, , C9 }
new_system! { S95, , C9 }
new_system! { S96, , C9 }
new_system! { S97, , C9 }
new_system! { S98, , C9 }
new_system! { S99, , C9 }

new_system! { S101, C2 C3 C4 C5 C6 C7 C8 C9 C1 C11 C12, C10 }
new_system! { S102, , C10 }
new_system! { S103, , C10 }
new_system! { S104, , C10 }
new_system! { S105, , C10 }
new_system! { S106, , C10 }
new_system! { S107, , C10 }
new_system! { S108, , C10 }
new_system! { S109, , C10 }
new_system! { S1010, , C10 }

new_system! { S111, C2 C3 C4 C5 C6 C7 C8 C9 C10 C1 C12, C11 }
new_system! { S112, , C11 }
new_system! { S113, , C11 }
new_system! { S114, , C11 }
new_system! { S115, , C11 }
new_system! { S116, , C11 }
new_system! { S117, , C11 }
new_system! { S118, , C11 }
new_system! { S119, , C11 }
new_system! { S1110, , C11 }
new_system! { S1111, , C11 }

new_system! { S121, C2 C3 C4 C5 C6 C7 C8 C9 C10 C11 C1, C12 }
new_system! { S122, , C12 }
new_system! { S123, , C12 }
new_system! { S124, , C12 }
new_system! { S125, , C12 }
new_system! { S126, , C12 }
new_system! { S127, , C12 }
new_system! { S128, , C12 }
new_system! { S129, , C12 }
new_system! { S1210, , C12 }
new_system! { S1211, , C12 }
new_system! { S1212, , C12 }
