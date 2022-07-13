use std::vec;
use crate::VirtualMachine;

pub struct WorldConfiguration {

}

pub struct World<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    vm: Vm,
    islands: Vec<crate::island::IslandData<RunResult, Vm>>,
}

impl<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> World<RunResult, Vm> {
    pub fn new(vm: Vm) -> World<RunResult, Vm> {
        World {
            vm,
            islands: vec![],
        }
    }

    /// Runs the next generation across all islands. 
    pub fn run_one_generation(&mut self) {
        for island in self.islands.iter_mut() {
            island.run_one_generation(&mut self.vm);
        }
    }
}