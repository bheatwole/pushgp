use crate::{Individual, VirtualMachine};

pub trait IslandCallbacks<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    /// Trait implementations can use this callback to configure any data that will apply to all individuals in this
    /// generation. Called once before any individuals are run. The default implementation does nothing.
    fn pre_generation_run(&mut self, _individuals: &[Individual<RunResult>]) {}

    /// Trait implementations can use this callback to perform any cleanup for this generation. Called once after all
    /// individuals are run. The default implementation does nothing.
    fn post_generation_run(&mut self, _individuals: &[Individual<RunResult>]) {}

    /// Run the virtual machine for a single individual. Called once for each individual on the island.
    ///
    /// A typical implementation might look like the following:
    /// ```ignore
    /// fn run_individual(&mut self, vm: &mut Vm, individual: &mut Individual<RunResult>) {
    ///     // Clear the stacks and defined functions from any previous runs
    ///     vm.clear();
    ///
    ///     // Setup this individuals' code
    ///     vm.set_code(individual.get_code().clone());
    ///
    ///     // Add any functions that this individual defined. This step would be skipped for simulations that do not
    ///     // use the 'Name' stack.
    ///     for (name, code) in individual.get_defined_names().iter() {
    ///         vm.name().define_name(name.clone(), code.clone());
    ///     }
    ///     
    ///     // Perform any other simulation or VM setup
    ///     // ...
    ///
    ///     // Run the vm for up to 10_000 instructions
    ///     vm.run(10_000);
    ///
    ///     // Calculate how fit this individual is, and store that value. This is the where each island will emphasize
    ///     // a different feature of an individual. One island may place a higher value on code size, another on
    ///     // 'winning' at any cost, another on 'not losing', etc
    ///     // individual.set_run_result(Some(my_calculate_fitness_for_island_x(vm)))
    /// }
    /// ```
    ///
    /// In a simulation where the inputs do not vary from generation to generation, the implementation may wish to check
    /// to see if a RunResult has already been saved for each individual, and skipping the function if already
    /// calculated in a previous run.
    fn run_individual(&mut self, vm: &mut Vm, individual: &mut Individual<RunResult>);

    /// Compare two individuals. The sort order is least fit to most fit. Called multiple times by the sorting algorithm
    /// after all individuals have been run.
    fn sort_individuals(&self, a: &Individual<RunResult>, b: &Individual<RunResult>) -> std::cmp::Ordering;
}
