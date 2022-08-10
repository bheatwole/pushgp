use crate::{Individual, IslandCallbacks, SelectionCurve, VirtualMachine};

pub struct IslandData<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    functions: Box<dyn IslandCallbacks<RunResult, Vm>>,
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
