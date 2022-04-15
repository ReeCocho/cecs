use bitvec::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::{
    collections::{HashMap, HashSet},
    ops::BitAnd,
    ops::{BitOr, BitXor, Not},
    ptr::NonNull,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{
    archetype::{archetypes::Archetypes, Archetype},
    component::filter::ComponentFilter,
    system::{GenericSystem, System},
    world::World,
};

/// Maximum number of systems allowed in a disaptcher.
pub const MAX_SYSTEMS: usize = 128;

/// Bits which represent a set of systems.
type SystemSet = BitArr!(for MAX_SYSTEMS);

/// The dispatcher is where systems exist and is responsible for scheduling systems optimally.
/// This is where the brunt of the logic for parallelization is going to go.
pub struct Dispatcher {
    systems: Vec<SystemStage>,
    thread_pool: ThreadPool,
    /// Maps each system to a `SystemSet` of compatible systems.
    compatibility: Vec<SystemSet>,
    /// Cache that maps a set of systems that we want to run in parallel with a subset of those
    /// systems that are actually compatible. Each bit in the `BitArr` represents a system.
    cache: HashMap<SystemSet, Vec<usize>>,
    cached_buffers: CachedBuffers,
}

/// Cached buffers so we don't have to reallocate.
#[derive(Default)]
struct CachedBuffers {
    bron_kerbosch: Vec<SystemSet>,
    to_remove: Vec<usize>,
    pending: HashSet<usize>,
    finished: Vec<usize>,
    running: HashSet<usize>,
}

/// Describes the state of a system in the dispatcher.
struct SystemStage {
    system: Box<dyn GenericSystem>,
    read_types: Archetype,
    write_types: Archetype,
    all_types: Archetype,
    /// Receiver that threads use to notify the main thread that the system has finished running.
    finished: Receiver<()>,
    /// Sender that threads use to notify the main thread that the system has finished running.
    thread_sender: Sender<()>,
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
    thread_sender: Sender<()>,
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
        let pending = &mut self.cached_buffers.pending;
        let finished = &mut self.cached_buffers.finished;
        let running = &mut self.cached_buffers.running;

        pending.clear();
        finished.clear();
        running.clear();

        // Setup: reset waiting counters. Systems with no dependencies are pending.
        for (i, system) in self.systems.iter_mut().enumerate() {
            system.waiting_on = system.dependency_count;

            if system.dependency_count == 0 {
                pending.insert(i);
            }
        }

        // Loop until all systems have finished
        while finished.len() != self.systems.len() {
            // Determine systems that have finished running
            let to_remove = &mut self.cached_buffers.to_remove;
            to_remove.clear();

            for system in running.iter() {
                let idx = *system;

                // Check to see if the system has finished running
                if self.systems[idx].finished.try_recv().is_err() {
                    continue;
                }

                to_remove.push(idx);
                finished.push(idx);

                // Notify dependencies of the completion
                // NOTE: Borrow checker bullsh*t means we can't iterate over `dependents` while
                // modifying `systems` because of mutable/immutable borrow.
                for i in 0..self.systems[idx].dependents.len() {
                    let dependent_idx = self.systems[idx].dependents[i];
                    let mut dependent = &mut self.systems[dependent_idx];

                    dependent.waiting_on -= 1;

                    // Move to pending if we aren't waiting anymore
                    if dependent.waiting_on == 0 {
                        pending.insert(dependent_idx);
                    }
                }
            }

            for idx in to_remove {
                running.remove(&idx);
            }

            // If there are no new pending systems, we loop
            if pending.is_empty() {
                continue;
            }

            // Create system sets
            let mut running_set: SystemSet = BitArray::ZERO;
            let mut pending_set: SystemSet = BitArray::ZERO;

            for idx in running.iter() {
                running_set.set(*idx, true);
            }

            for idx in pending.iter() {
                pending_set.set(*idx, true);
            }

            // Check if we've seen this combo already in the cache
            let all_systems = running_set.bitor(pending_set);

            let to_run = if let Some(result) = self.cache.get(&all_systems) {
                result
            }
            // Not in the cache. Need to perform Bron-Kerbosch
            else {
                let mut max_cliques = &mut self.cached_buffers.bron_kerbosch;
                max_cliques.clear();

                bron_kerbosch(
                    running_set,
                    pending_set,
                    BitArray::ZERO,
                    &self.compatibility,
                    &mut max_cliques,
                );

                // Pick the maximum amongst all the maximal cliques
                let mut result = if max_cliques.is_empty() {
                    BitArray::ZERO
                } else {
                    // Find the maximum amongst all the cliques
                    let mut max = 0;
                    let mut max_len = max_cliques[0].count_ones();

                    for (i, clique) in max_cliques.iter().enumerate().skip(1) {
                        if clique.count_ones() > max_len {
                            max = i;
                            max_len = clique.count_ones();
                        }
                    }

                    max_cliques[max]
                };

                // Get rid of the running systems
                result = result.bitxor(running_set);
                let mut to_cache = Vec::with_capacity(result.count_ones());
                for i in result.iter_ones() {
                    to_cache.push(i);
                }

                // Add to the cache
                self.cache.insert(all_systems, to_cache);
                self.cache.get(&all_systems).unwrap()
            };

            // Send all compatible systems to the thread pool
            for system in to_run {
                let idx = *system;

                // Ignore if already running
                if running.contains(&idx) {
                    continue;
                }

                running.insert(idx);
                pending.remove(&idx);

                let packet = SystemPacket {
                    system: unsafe {
                        NonNull::new_unchecked(self.systems[idx].system.as_mut() as *mut _)
                    },
                    archetypes: (&world.archetypes) as *const _,
                    thread_sender: self.systems[idx].thread_sender.clone(),
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
                    packet.thread_sender.send(()).unwrap();
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
    pub fn with_system<S: System + 'static>(
        &mut self,
        system: S,
        dependencies: &[SystemId],
    ) -> SystemId {
        assert_ne!(self.systems.len(), MAX_SYSTEMS);

        let id = SystemId(self.systems.len());

        // Notify all dependencies of the new dependent
        for dependency in dependencies {
            self.systems[dependency.0].dependents.push(id.0);
        }

        // Create channels
        let (thread_sender, finished) = crossbeam_channel::bounded(1);

        // Add the stage
        self.systems.push(SystemStage {
            system: Box::new(system),
            read_types: S::Components::read_archetype(),
            write_types: S::Components::write_archetype(),
            all_types: S::Components::archetype(),
            dependency_count: dependencies.len(),
            waiting_on: dependencies.len(),
            dependents: Vec::default(),
            thread_sender,
            finished,
        });

        id
    }

    pub fn build(self) -> Dispatcher {
        // Determine which systems are compatible with which
        let mut compatibility = Vec::with_capacity(self.systems.len());

        for (i, system) in self.systems.iter().enumerate() {
            let mut compatible: SystemSet = BitArray::ZERO;

            for (j, other_system) in self.systems.iter().enumerate() {
                // Write archetypes must not overlap (also, we are compatible with ourselves)
                if (j != i)
                    && (system.all_types.any_of(&other_system.write_types)
                        || other_system.all_types.any_of(&system.write_types))
                {
                    continue;
                }

                compatible.set(j, true);
            }

            compatibility.push(compatible);
        }

        Dispatcher {
            systems: self.systems,
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(self.thread_count)
                .build()
                .unwrap(),
            compatibility,
            cache: HashMap::default(),
            cached_buffers: CachedBuffers::default(),
        }
    }
}

/// Helper function that performs the Bron-Kerbosch algorithm.
fn bron_kerbosch(
    r: SystemSet,
    mut p: SystemSet,
    mut x: SystemSet,
    compatibility: &[SystemSet],
    out: &mut Vec<SystemSet>,
) {
    if p.not_any() && x.not_any() {
        out.push(r);
        return;
    }

    let px = p.bitor(x);
    let pivot = px.first_one().unwrap();

    let mut nh_pivot = compatibility[pivot].clone();
    nh_pivot.set(pivot, false);

    let p_removing_nh_pivot = p & (nh_pivot.not());

    for v in p_removing_nh_pivot.iter_ones() {
        let mut nh_v = compatibility[v].clone();
        nh_v.set(v, false);

        let mut new_r = r.clone();
        new_r.set(v, true);

        let new_p = p.clone().bitand(nh_v);

        let new_x = x.clone().bitand(nh_v);

        bron_kerbosch(new_r, new_p, new_x, compatibility, out);

        p.set(v, false);
        x.set(v, true);
    }
}
