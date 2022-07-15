use std::vec;
use crate::VirtualMachine;

pub struct WorldConfiguration {
    /// After this many generations across all islands, some of the individual will migrate to new islands. Set to zero
    /// to disable automatic migrations.
    generations_between_migrations: usize,
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
        World {
            vm,
            config,
            islands: vec![],
            generations_remaining_before_migration,
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

    pub fn migrate_individuals_between_islands(&mut self) {

    }
}