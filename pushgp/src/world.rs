use crate::{Island, MigrationAlgorithm, SelectionCurve, VirtualMachine, IslandCallbacks};
use std::vec;

pub type IslandId = usize;

pub struct WorldConfiguration {
    /// After this many generations across all islands, some of the individual will migrate to new islands. Set to zero
    /// to disable automatic migrations.
    generations_between_migrations: usize,

    /// When it is time for a migration, a new island will be selected for the individual according to the specified
    /// algorithm.
    migration_algorithm: MigrationAlgorithm,

    /// If false, individuals selected for migration are removed from their home island. If true, the selected
    /// individuals are cloned and the clone is moved. The default is true
    clone_migrated_individuals: bool,

    /// The SelectionCurve that will be used when choosing which individual will participate in migration. The default
    /// is PreferenceForFit.
    select_for_migration: SelectionCurve,

    /// The SelectionCurve that will be used when choosing which individuals to permanently remove from an island
    /// because there is not enough room for newly migrating individuals. Default is StrongPreferenceForUnfit
    select_for_overpopulation_removal: SelectionCurve,
}

impl Default for WorldConfiguration {
    fn default() -> Self {
        WorldConfiguration {
            generations_between_migrations: 10,
            migration_algorithm: MigrationAlgorithm::Circular,
            clone_migrated_individuals: true,
            select_for_migration: SelectionCurve::PreferenceForFit,
            select_for_overpopulation_removal: SelectionCurve::StrongPreferenceForUnfit,
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

    pub fn migrate_individuals_between_islands(&mut self) {}
}
