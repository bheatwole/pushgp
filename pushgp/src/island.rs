use crate::{Individual, VirtualMachine};

pub trait Island<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    fn pre_generation_run(&mut self, individuals: &[Individual<RunResult, Vm>]);
    fn post_generation_run(&mut self, individuals: &[Individual<RunResult, Vm>]);

    /// Run the virtual machine for a single individual.
    fn run_individual(&mut self, vm: &mut Vm, individual: &mut Individual<RunResult, Vm>);

    /// Compare two individuals. The sort order is least fit to most fit.
    fn sort_individuals(&self, a: &Individual<RunResult, Vm>, b: &Individual<RunResult, Vm>) -> std::cmp::Ordering;
}

pub struct IslandData<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    functions: Box<dyn Island<RunResult, Vm>>,
    individuals: Vec<Individual<RunResult, Vm>>,
}

impl<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> IslandData<RunResult, Vm> {
    pub fn run_one_generation(&mut self, vm: &mut Vm) {
        // Allow the island to set up for all runs
        self.functions.pre_generation_run(&self.individuals);

        // Run each individual
        for individual in self.individuals.iter_mut() {
            self.functions.run_individual(vm, individual);
        }

        // Allow the island to before any cleanup or group analysis tasks
        self.functions.post_generation_run(&self.individuals);

        // Sort the individuals
        self.sort_individuals();
    }

    /// Sorts the individuals by calling the sorter function.
    fn sort_individuals(&mut self) {
        // It is useful to swap the Vec into a local variable to avoid borrow-checking issues
        let mut local_individuals = vec![];
        std::mem::swap(&mut self.individuals, &mut local_individuals);
        local_individuals.sort_by(|a, b| self.functions.sort_individuals(a, b));
        std::mem::swap(&mut self.individuals, &mut local_individuals);
    }
}