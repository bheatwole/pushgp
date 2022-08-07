use crate::{Individual, SelectionCurve, VirtualMachine};

pub trait Island<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    /// Trait implementations can use this callback to configure any data that will apply to all individuals in this
    /// generation. Called once before any individuals are run. The default implementation does nothing.
    fn pre_generation_run(&mut self, _individuals: &[Individual<RunResult, Vm>]) {}

    /// Trait implementations can use this callback to perform any cleanup for this generation. Called once after all
    /// individuals are run. The default implementation does nothing.
    fn post_generation_run(&mut self, _individuals: &[Individual<RunResult, Vm>]) {}

    /// Run the virtual machine for a single individual. Called once for each individual on the island.
    ///
    /// A typical implementation might look like the following:
    /// ```
    /// fn run_individual(&mut self, vm: &mut Vm, individual: &mut Individual<RunResult, Vm>) {
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
    fn run_individual(&mut self, vm: &mut Vm, individual: &mut Individual<RunResult, Vm>);

    /// Compare two individuals. The sort order is least fit to most fit. Called multiple times by the sorting algorithm
    /// after all individuals have been run.
    fn sort_individuals(&self, a: &Individual<RunResult, Vm>, b: &Individual<RunResult, Vm>) -> std::cmp::Ordering;
}

pub struct IslandData<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    functions: Box<dyn Island<RunResult, Vm>>,
    individuals: Vec<Individual<RunResult, Vm>>,
    future: Vec<Individual<RunResult, Vm>>,
}

impl<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> IslandData<RunResult, Vm> {
    /// Uses the specified VM to run one generation of individuals. Calls all of the user-supplied functions from the
    /// `Island` trait.
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
        // It is useful to swap the Vec into a local variable to avoid borrow-checking issues during the sort
        let mut local_individuals = vec![];
        std::mem::swap(&mut self.individuals, &mut local_individuals);
        local_individuals.sort_by(|a, b| self.functions.sort_individuals(a, b));
        std::mem::swap(&mut self.individuals, &mut local_individuals);
    }

    /// Returns the current number of individuals on the island.
    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    /// Returns the number of individuals in the next generation
    pub fn len_future_generation(&self) -> usize {
        self.future.len()
    }

    /// Permanently removes all of the current generation and sets the future generation as the current generation.
    pub fn advance_generation(&mut self) {
        self.individuals.clear();
        std::mem::swap(&mut self.individuals, &mut self.future);
    }

    /// Select one individual from the island according to the specified SelectionCurve and borrow it.
    /// Returns the individual borrowed or None if the population is zero
    pub fn select_one_individual<R: rand::Rng>(
        &self,
        curve: SelectionCurve,
        rng: &mut R,
    ) -> Option<&Individual<RunResult, Vm>> {
        let max = self.individuals.len();
        if max == 0 {
            None
        } else {
            self.individuals.get(curve.pick_one_index(rng, max))
        }
    }

    /// Select one individual from the island according to the specified SelectionCurve and remove it permanently.
    /// Returns the individual removed or None if the population is zero
    pub fn select_and_remove_one_individual<R: rand::Rng>(
        &mut self,
        curve: SelectionCurve,
        rng: &mut R,
    ) -> Option<Individual<RunResult, Vm>> {
        let max = self.individuals.len();
        if max == 0 {
            None
        } else {
            Some(self.individuals.remove(curve.pick_one_index(rng, max)))
        }
    }

    /// Adds an individual to the future generation
    pub fn add_individual_to_future_generation(&mut self, individual: Individual<RunResult, Vm>) {
        self.future.push(individual);
    }
}
