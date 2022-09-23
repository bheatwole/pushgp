use crate::*;
use fnv::FnvHashMap;
use rand::{prelude::SliceRandom, Rng};
use std::vec;

pub type IslandId = usize;

const RETRIES: usize = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct WorldConfiguration {
    /// The number of individuals on each island. Before running a generation, the island will be filled with the
    /// children of genetic selection if there was a previous generation, or new random individuals if there was no
    /// previous generation.
    pub individuals_per_island: usize,

    /// The number of individuals whose code will be copied as-is to the next generation. This can help preserve highly
    /// fit code. Set to zero to disable elitism. ref https://en.wikipedia.org/wiki/Genetic_algorithm#Elitism
    pub elite_individuals_per_generation: usize,

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

    /// The SelectionCurve used when choosing an elite individual to preserve for the next generation. The default is
    /// StrongPreferenceForFit.
    pub select_as_elite: SelectionCurve,

    /// Determine how the world runs with regards to multi-threading. Placeholder: currently multi-threading is not
    /// implemented
    pub threading_model: ThreadingModel,
}

impl Default for WorldConfiguration {
    fn default() -> Self {
        WorldConfiguration {
            individuals_per_island: 100,
            elite_individuals_per_generation: 2,
            generations_between_migrations: 10,
            number_of_individuals_migrating: 10,
            migration_algorithm: MigrationAlgorithm::Circular,
            clone_migrated_individuals: true,
            select_for_migration: SelectionCurve::PreferenceForFit,
            select_as_parent: SelectionCurve::PreferenceForFit,
            select_as_elite: SelectionCurve::StrongPreferenceForFit,
            threading_model: ThreadingModel::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct World<R: RunResult, Vm: VirtualMachine> {
    vm: Vm,
    config: WorldConfiguration,
    islands: Vec<Island<R, Vm>>,
    generations_remaining_before_migration: usize,
}

impl<R: RunResult, Vm: VirtualMachine> World<R, Vm> {
    pub fn new(vm: Vm, config: WorldConfiguration) -> World<R, Vm> {
        let generations_remaining_before_migration = config.generations_between_migrations;
        World { vm, config, islands: vec![], generations_remaining_before_migration }
    }

    pub fn get_vm(&self) -> &Vm {
        &self.vm
    }

    pub fn get_vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }

    /// Adds a new island to the World that will use the specified callbacks to perform the various individual
    /// processing tasks required during its lifetime
    pub fn create_island(&mut self, callbacks: Box<dyn IslandCallbacks<R, Vm>>) -> IslandId {
        let id = self.islands.len();
        self.islands.push(Island::new(callbacks));

        id
    }

    /// Returns the total number of islands
    pub fn get_number_of_islands(&self) -> usize {
        self.islands.len()
    }

    /// Borrows an island by the specified ID
    pub fn get_island(&self, id: IslandId) -> Option<&Island<R, Vm>> {
        self.islands.get(id)
    }

    /// Mutably borrows an island by the specified ID
    pub fn get_island_mut(&mut self, id: IslandId) -> Option<&mut Island<R, Vm>> {
        self.islands.get_mut(id)
    }

    /// Removes all individuals from all islands
    pub fn reset_all_islands(&mut self) {
        for island in self.islands.iter_mut() {
            island.clear();
        }
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
            let mut elite_remaining = self.config.elite_individuals_per_generation;
            while island.len_future_generation() < self.config.individuals_per_island {
                self.vm.engine_mut().clear();

                let next = if island.len() == 0 {
                    run_with_retry(|| {
                        let code = self.vm.engine_mut().rand_code(None)?;
                        Ok(Individual::new(code, FnvHashMap::default(), None))
                    }).expect("Unable to generate new code that doesn't use excessive number of Code in list. Check configuration.")
                } else {
                    if elite_remaining > 0 {
                        elite_remaining -= 1;
                        island.select_one_individual(self.config.select_as_elite, self.vm.get_rng()).unwrap().clone()
                    } else {
                        run_with_retry(|| {
                            let left =
                                island.select_one_individual(self.config.select_as_parent, self.vm.get_rng()).unwrap();
                            let right =
                                island.select_one_individual(self.config.select_as_parent, self.vm.get_rng()).unwrap();
                            self.vm.engine_mut().rand_child(left, right)
                        }).expect("Unable to generate child that doesn't use excessive number of Code in list. Check configuration.")
                    }
                };
                island.add_individual_to_future_generation(next);
            }

            // Now that the future generation is full, make it the current generation
            island.advance_generation();
        }
    }

    /// Runs generations until the specified function returns false
    pub fn run_generations_while<While>(&mut self, mut while_fn: While)
    where
        While: FnMut(&World<R, Vm>) -> bool,
    {
        // Always run at least one generation
        let mut running = true;
        while running {
            self.fill_all_islands();
            self.run_one_generation();
            running = while_fn(self);
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
                    let distances = World::<R, Vm>::distances_to_next_island(&island_order[..]);
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
        let migrating: Individual<R> = if self.config.clone_migrated_individuals {
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

    /// Generates 10 random individuals per island per run. The instructions in the most fit and least fit individual
    /// are counted and a determination made as to which instructions most benefit, and which cause the most harm, to
    /// the population as a whole.
    ///
    /// This will call `clear` on all islands, so do not run after starting normal generations.
    pub fn heuristically_calculate_instruction_weights(&mut self, runs: usize) -> FnvHashMap<&'static str, u8> {
        let mut most_fit_instructions: FnvHashMap<&'static str, usize> = FnvHashMap::default();
        let mut least_fit_instructions: FnvHashMap<&'static str, usize> = FnvHashMap::default();

        // Setup a config for this algorithm and swap it in for the original configuration
        let mut swap_config = WorldConfiguration {
            individuals_per_island: 10,
            elite_individuals_per_generation: 0,
            generations_between_migrations: 0,
            number_of_individuals_migrating: 0,
            migration_algorithm: MigrationAlgorithm::Circular,
            clone_migrated_individuals: true,
            select_for_migration: SelectionCurve::Fair,
            select_as_parent: SelectionCurve::Fair,
            select_as_elite: SelectionCurve::Fair,
            threading_model: ThreadingModel::None,
        };
        std::mem::swap(&mut self.config, &mut swap_config);

        // Start our runs
        for _ in 0..runs {
            // Run the initial generation on all islands
            self.reset_all_islands();
            self.fill_all_islands();
            self.run_one_generation();

            // Update the instruction count from the most fit and least fit individuals
            for island in self.islands.iter() {
                let code = island.most_fit_individual().unwrap().get_code();
                self.update_instruction_count(&mut most_fit_instructions, code);
                let code = island.least_fit_individual().unwrap().get_code();
                self.update_instruction_count(&mut least_fit_instructions, code);
            }
        }

        // Reset the islands one last time and restore the original config
        self.reset_all_islands();
        std::mem::swap(&mut self.config, &mut swap_config);

        // Determine the max count for any instruction
        let most_fit_max = most_fit_instructions.iter().fold(0, |acc, (_key, count)| acc + count);
        let least_fit_max = least_fit_instructions.iter().fold(0, |acc, (_key, count)| acc + count);

        // Loop through every instruction that the VM has and calculate the new weight. An instruction that appears
        // more than twice as often in the least fit individuals will have a weight of zero. An instruction that appears
        // only in the most fit individuals will have a weight of 255. Instructions that do not appear at all will be
        // skipped (and get whatever the user decides is the default weight).
        let mut weights = FnvHashMap::default();
        let all_instructions = self.vm.engine().get_weights().get_instruction_names();
        for instruction in all_instructions {
            let most_fit_frequency = instruction_frequency(instruction, &most_fit_instructions, most_fit_max);
            let least_fit_frequency = instruction_frequency(instruction, &least_fit_instructions, least_fit_max) * 0.5;
            if most_fit_frequency > 0.0 || least_fit_frequency > 0.0 {
                // This instruction appeared at least once, so we should calculate its effect
                let total_frequency = most_fit_frequency - least_fit_frequency;
                if total_frequency <= 0.0 {
                    // This instruction had a very negative effect, don't use it
                    weights.insert(instruction, 0);
                } else {
                    let weight: u8 = (total_frequency * 255.0).floor() as u8;
                    weights.insert(instruction, weight);
                }
            }
        }

        weights
    }

    fn update_instruction_count(&self, instructions: &mut FnvHashMap<&'static str, usize>, code: &Code) {
        for atom in code.extract_atoms().iter() {
            let name = self.vm.name_for_opcode(atom.get_opcode()).unwrap();
            *(instructions.entry(name).or_insert(0)) += 1;
        }
    }
}

// The frequency of an instruction is how often it appears relative to the instruction that appears the most
fn instruction_frequency(search_for: &str, instructions: &FnvHashMap<&'static str, usize>, max: usize) -> f64 {
    let count = instructions.get(search_for).unwrap_or(&0);
    (*count) as f64 / max as f64
}

fn run_with_retry<R: RunResult, F: FnMut() -> Result<Individual<R>, ExecutionError>>(
    mut func: F,
) -> Option<Individual<R>> {
    let mut retries = RETRIES;
    let mut code = None;
    while retries > 0 {
        retries -= 1;
        match func() {
            Ok(rand_code) => {
                code = Some(rand_code);
                break;
            }
            Err(_) => {}
        }
    }

    code
}
