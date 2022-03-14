use crate::{
    system::{GenericSystem, System},
    world::World,
};

/// The dispatcher is where systems exist and is responsible for scheduling systems optimally.
/// This is where the brunt of the logic for parallelization is going to go.
pub struct Dispatcher {
    systems: Vec<Box<dyn GenericSystem>>,
}

/// If you are unfamiliar with the builder pattern, considering taking a look at this link:
/// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
pub struct DispatcherBuilder {
    dispatcher: Dispatcher,
}

impl Dispatcher {
    #[inline]
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::new()
    }

    /// Runs one tick of every system within the dispatcher using a given world.
    pub fn run(&mut self, world: &mut World) {
        // TODO: This runs the systems in serial. It should be parallelized.
        for system in &mut self.systems {
            system.generic_tick(&world.archetypes);
        }
    }
}

impl Default for DispatcherBuilder {
    fn default() -> Self {
        Self {
            dispatcher: Dispatcher {
                systems: Vec::default(),
            },
        }
    }
}

impl DispatcherBuilder {
    #[inline]
    pub fn new() -> Self {
        DispatcherBuilder::default()
    }

    /// Adds a new system to the dispatcher. We should be able to
    pub fn with_system(mut self, system: impl System + 'static) -> Self {
        self.dispatcher.systems.push(Box::new(system));
        self
    }

    pub fn build(self) -> Dispatcher {
        self.dispatcher
    }
}
