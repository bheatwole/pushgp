use crate::{MigrationAlgorithm, SelectionCurve, VirtualMachine};
use std::vec;

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
    islands: Vec<crate::island::IslandData<RunResult, Vm>>,
    generations_remaining_before_migration: usize,
}

impl<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> World<RunResult, Vm> {
    pub fn new(vm: Vm, config: WorldConfiguration) -> World<RunResult, Vm> {
        let generations_remaining_before_migration = config.generations_between_migrations;
        World { vm, config, islands: vec![], generations_remaining_before_migration }
    }

    // TODO: rename Island as IslandCallbacks and IslandData as Island
    // TODO: create methods for adding islands to and getting islands from the world. Use index based identities

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
