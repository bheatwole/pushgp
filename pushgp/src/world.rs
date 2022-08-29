use crate::{Individual, Island, IslandCallbacks, MigrationAlgorithm, SelectionCurve, VirtualMachine};
use fnv::FnvHashMap;
use rand::{prelude::SliceRandom, Rng};
use std::vec;

pub type IslandId = usize;

pub struct WorldConfiguration {
    /// The number of individuals on each island. Before running a generation, the island will be filled with the
    /// children of genetic selection if there was a previous generation, or new random individuals if there was no
    /// previous generation.
    pub individuals_per_island: usize,

    /// After this many generations across all islands, some of the individual will migrate to new islands. Set to zero
    /// to disable automatic migrations.
    pub generations_between_migrations: usize,

    /// The number of individuals that will migrate from one island to another.
    pub number_of_individuals_migrating: usize,

    /// When it is time for a migration, a new island will be selected for the individual according to the specified
    /// algorithm.
    pub migration_algorithm: MigrationAlgorithm,

    /// If false, individuals selected for migration are removed from their home island. If true, the selected
    /// individuals are cloned and the clone is moved. The default is true
    pub clone_migrated_individuals: bool,

    /// The SelectionCurve that will be used when choosing which individual will participate in migration. The default
    /// is PreferenceForFit.
    pub select_for_migration: SelectionCurve,

    /// The SelectionCurve that will be used when choosing a fit parent for genetic operations. The default is
    /// PreferenceForFit.
    pub select_as_parent: SelectionCurve,
}

impl Default for WorldConfiguration {
    fn default() -> Self {
        WorldConfiguration {
            individuals_per_island: 100,
            generations_between_migrations: 10,
            number_of_individuals_migrating: 10,
            migration_algorithm: MigrationAlgorithm::Circular,
            clone_migrated_individuals: true,
            select_for_migration: SelectionCurve::PreferenceForFit,
            select_as_parent: SelectionCurve::PreferenceForFit,
        }
    }
}

pub struct World<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    vm: Vm,
    config: WorldConfiguration,
    islands: Vec<Island<RunResult, Vm>>,
    generations_remaining_before_migration: usize,
}

impl<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> World<RunResult, Vm> {
    pub fn new(vm: Vm, config: WorldConfiguration) -> World<RunResult, Vm> {
        let generations_remaining_before_migration = config.generations_between_migrations;
        World { vm, config, islands: vec![], generations_remaining_before_migration }
    }

    /// Adds a new island to the World that will use the specified callbacks to perform the various individual
    /// processing tasks required during its lifetime
    pub fn create_island(&mut self, callbacks: Box<dyn IslandCallbacks<RunResult, Vm>>) -> IslandId {
        let id = self.islands.len();
        self.islands.push(Island::new(callbacks));

        id
    }

    /// Borrows an island by the specified ID
    pub fn get_island(&self, id: IslandId) -> Option<&Island<RunResult, Vm>> {
        self.islands.get(id)
    }

    /// Mutably borrows an island by the specified ID
    pub fn get_island_mut(&mut self, id: IslandId) -> Option<&mut Island<RunResult, Vm>> {
        self.islands.get_mut(id)
    }

    /// Runs the next generation across all islands.
    pub fn run_one_generation(&mut self) {
        for island in self.islands.iter_mut() {
            island.run_one_generation(&mut self.vm);
        }

        // See if it is time for a migration
        if self.config.generations_between_migrations > 0 {
            self.generations_remaining_before_migration -= 1;
            if self.generations_remaining_before_migration == 0 {
                self.migrate_individuals_between_islands();
                self.generations_remaining_before_migration = self.config.generations_between_migrations;
            }
        }
    }

    /// Fills all islands with the children of the genetic algorithm, or with random individuals if there was no
    /// previous generation from which to draw upon.
    pub fn fill_all_islands(&mut self) {
        for island in self.islands.iter_mut() {
            while island.len_future_generation() < self.config.individuals_per_island {
                self.vm.engine_mut().clear();

                if island.len() == 0 {
                    let code = self.vm.engine_mut().rand_code(None);
                    island.add_individual_to_future_generation(Individual::new(code, FnvHashMap::default(), None));
                } else {
                    let left = island.select_one_individual(self.config.select_as_parent, self.vm.get_rng()).unwrap();
                    let right = island.select_one_individual(self.config.select_as_parent, self.vm.get_rng()).unwrap();
                    let child = self.vm.engine_mut().rand_child(left, right);
                    island.add_individual_to_future_generation(child);
                }
            }
        }
    }

    /// Runs generations until the specified function returns false
    pub fn run_generations_until<Until>(&mut self, mut until: Until)
    where
        Until: FnMut(&World<RunResult, Vm>) -> bool,
    {
        // Always run at least one generation
        let mut running = true;
        while running {
            self.fill_all_islands();
            self.run_one_generation();
            running = until(self);
        }
    }

    pub fn migrate_individuals_between_islands(&mut self) {
        let island_len = self.islands.len();

        // It only makes sense to migrate if there are at least two islands
        if island_len > 1 {
            match self.config.migration_algorithm {
                MigrationAlgorithm::Circular => self.migrate_all_islands_circular_n(1),
                MigrationAlgorithm::Cyclical(n) => self.migrate_all_islands_circular_n(n),
                MigrationAlgorithm::Incremental(n) => {
                    self.migrate_all_islands_circular_n(n);

                    // Increment 'n'. An 'n' of zero makes no sense, so when it gets there use '1' instead.
                    let mut next_n = self.island_at_distance(0, n + 1);
                    if next_n == 0 {
                        next_n = 1
                    }
                    self.config.migration_algorithm = MigrationAlgorithm::Incremental(next_n);
                }
                MigrationAlgorithm::RandomCircular => {
                    // Define a new order of islands and calculate the distance to the next island in this new order.
                    // For example, if there are 7 islands and the order starts with 2, 3: the first distance is 1.
                    // However if the order starts with 3, 2: the first distance is 6
                    //
                    // This algorithm achieves the desired goal of having individuals from each island migrate together
                    // to another random island, and each island is the source and destination exactly once.
                    let island_order = self.random_island_order();
                    let distances = World::<RunResult, Vm>::distances_to_next_island(&island_order[..]);
                    for (source_id, n) in std::iter::zip(island_order, distances) {
                        self.migrate_one_island_circular_n(source_id, n);
                    }
                }
                MigrationAlgorithm::CompletelyRandom => {
                    let len = self.islands.len();

                    // For each migrating individual on each island, pick a random destination that is not the same
                    // island and migrate there.
                    for source_island_id in 0..len {
                        for _ in 0..self.config.number_of_individuals_migrating {
                            let mut destination_island_id = source_island_id;
                            while source_island_id != destination_island_id {
                                destination_island_id = self.vm.get_rng().gen_range(0..len);
                            }
                            self.migrate_one_individual_from_island_to_island(source_island_id, destination_island_id);
                        }
                    }
                }
            }
        }
    }

    fn migrate_one_individual_from_island_to_island(
        &mut self,
        source_island_id: IslandId,
        destination_island_id: IslandId,
    ) {
        let curve = self.config.select_for_migration;

        // Get the migrating individual from the source island
        let source_island = self.islands.get_mut(source_island_id).unwrap();
        let migrating: Individual<RunResult, Vm> = if self.config.clone_migrated_individuals {
            source_island.select_one_individual(curve, self.vm.get_rng()).unwrap().clone()
        } else {
            source_island.select_and_remove_one_individual(curve, self.vm.get_rng()).unwrap()
        };

        // Add it to the destination island
        let destination_island = self.islands.get_mut(destination_island_id).unwrap();
        destination_island.add_individual_to_future_generation(migrating);
    }

    // Calculates the ID of the island at a specific distance from the source. Wraps around when we get to the end of
    // the list.
    fn island_at_distance(&self, source_id: IslandId, distance: usize) -> IslandId {
        (source_id + distance) % self.islands.len()
    }

    fn migrate_all_islands_circular_n(&mut self, n: usize) {
        for source_island_id in 0..self.islands.len() {
            self.migrate_one_island_circular_n(source_island_id, n);
        }
    }

    fn migrate_one_island_circular_n(&mut self, source_island_id: IslandId, n: usize) {
        let destination_island_id = self.island_at_distance(source_island_id, n);
        for _ in 0..self.config.number_of_individuals_migrating {
            self.migrate_one_individual_from_island_to_island(source_island_id, destination_island_id);
        }
    }

    // Creates a Vec containing the source_id of each island exactly one time
    fn random_island_order(&mut self) -> Vec<IslandId> {
        let mut island_ids: Vec<IslandId> = (0..self.islands.len()).collect();
        island_ids.shuffle(self.vm.get_rng());

        island_ids
    }

    // Creates a Vec containing the distance to the previous island in the list for every entry in the parameter. The
    // distance for the first entry wraps around to the last item.
    fn distances_to_next_island(island_id: &[IslandId]) -> Vec<IslandId> {
        let len = island_id.len();
        let mut distances = Vec::with_capacity(len);
        let mut previous_source_id = island_id.last().unwrap();
        for source_id in island_id.iter() {
            let distance = ((previous_source_id + len) - source_id) % len;
            distances.push(distance);
            previous_source_id = source_id;
        }

        distances
    }
}
