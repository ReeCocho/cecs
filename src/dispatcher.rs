use rayon::{ThreadPool, ThreadPoolBuilder};
use std::{
    collections::HashSet,
    ptr::NonNull,
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    archetype::archetypes::Archetypes,
    system::{GenericSystem, System},
    world::World,
};

/// The dispatcher is where systems exist and is responsible for scheduling systems optimally.
/// This is where the brunt of the logic for parallelization is going to go.
pub struct Dispatcher {
    systems: Vec<SystemStage>,
    thread_pool: ThreadPool,
    /// Set of running systems.
    running: HashSet<usize>,
    /// Receiver that threads use to notify the main thread that a system has finished running.
    finished: Receiver<usize>,
    /// Sender that threads use to notify the main thread that a system has finished running.
    thread_sender: Sender<usize>,
}

/// Describes the state of a system in the dispatcher.
struct SystemStage {
    system: Box<dyn GenericSystem>,
    /// Number of dependencies this system has.
    dependency_count: usize,
    /// Number of dependencies the system is waiting on currently.
    waiting_on: usize,
    /// Indices of systems that are dependent on us.
    dependents: Vec<usize>,
}

/// Description for a thread of a system to run.
struct SystemPacket {
    /// System to run.
    system: NonNull<dyn GenericSystem>,
    /// Archetypes the system must use.
    archetypes: *const Archetypes,
    /// Sender that threads use to notify the main thread that a system has finished running.
    thread_sender: Sender<usize>,
    /// Index of the system to return when the system finishes running.
    idx: usize,
}

unsafe impl Send for SystemPacket {}

/// If you are unfamiliar with the builder pattern, considering taking a look at this link:
/// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
pub struct DispatcherBuilder {
    systems: Vec<SystemStage>,
    thread_count: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemId(usize);

impl Dispatcher {
    #[inline]
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::new()
    }

    /// Runs one tick of every system within the dispatcher using a given world.
    pub fn run(&mut self, world: &mut World) {
        let mut pending = Vec::default();
        let mut finished = Vec::default();
        let mut to_run = Vec::<usize>::default();

        // Setup: reset waiting counters. Systems with no dependencies are pending.
        for (i, system) in self.systems.iter_mut().enumerate() {
            system.waiting_on = system.dependency_count;

            if system.dependency_count == 0 {
                pending.push(i);
            }
        }

        // Loop until we have no pending systems
        while !pending.is_empty() {
            // Determine systems that have finished running
            finished.clear();

            for system_idx in self.finished.try_iter() {
                self.running.remove(&system_idx);
                finished.push(system_idx);

                // Notify dependencies of the completion
                // NOTE: Borrow checker bullsh*t means we can't iterate over `dependents` while
                // modifying `systems` because of mutable/immutable borrow.
                for i in 0..self.systems[system_idx].dependents.len() {
                    let dependent_idx = self.systems[system_idx].dependents[i];
                    let mut dependent = &mut self.systems[dependent_idx];

                    dependent.waiting_on -= 1;

                    // Move to pending if we aren't waiting anymore
                    if dependent.waiting_on == 0 {
                        pending.push(dependent_idx);
                    }
                }
            }

            // TODO: Bron-Kerbosch and caching

            // Send all compatible systems to the thread pool
            for idx in to_run.drain(..) {
                self.running.insert(idx);

                let packet = SystemPacket {
                    system: unsafe {
                        NonNull::new_unchecked(self.systems[idx].system.as_mut() as *mut _)
                    },
                    archetypes: (&world.archetypes) as *const _,
                    thread_sender: self.thread_sender.clone(),
                    idx,
                };

                self.thread_pool.spawn(move || unsafe {
                    // Move packet to the thread
                    let mut packet = packet;

                    // Convert back to reference
                    let archetypes = packet.archetypes.as_ref().unwrap();

                    // Run the system
                    packet.system.as_mut().generic_tick(archetypes);

                    // Notify the main thread that the system has completed
                    packet.thread_sender.send(packet.idx).unwrap();
                });
            }
        }
    }
}

impl Default for DispatcherBuilder {
    fn default() -> Self {
        Self {
            systems: Vec::default(),
            thread_count: 1,
        }
    }
}

impl DispatcherBuilder {
    #[inline]
    pub fn new() -> Self {
        DispatcherBuilder::default()
    }

    pub fn thread_count(mut self, thread_count: usize) -> Self {
        self.thread_count = thread_count;
        self
    }

    /// Adds a new system to the dispatcher. Returns a unique ID for the system to define
    /// dependencies.
    pub fn with_system(
        &mut self,
        system: impl System + 'static,
        dependencies: &[SystemId],
    ) -> SystemId {
        let id = SystemId(self.systems.len());

        // Notify all dependencies of the new dependent
        for dependency in dependencies {
            self.systems[dependency.0].dependents.push(id.0);
        }

        // Add the stage
        self.systems.push(SystemStage {
            system: Box::new(system),
            dependency_count: dependencies.len(),
            waiting_on: dependencies.len(),
            dependents: Vec::default(),
        });

        id
    }

    pub fn build(self) -> Dispatcher {
        let (thread_sender, finished) = std::sync::mpsc::channel();

        Dispatcher {
            systems: self.systems,
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(self.thread_count)
                .build()
                .expect("unable to create thread pool"),
            thread_sender,
            finished,
            running: HashSet::default(),
        }
    }
}
