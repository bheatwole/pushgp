use crate::{Individual, IslandCallbacks, RunResult, SelectionCurve, VirtualMachine};

#[derive(Debug)]
pub struct Island<R: RunResult, Vm: VirtualMachine> {
    functions: Box<dyn IslandCallbacks<R, Vm>>,
    individuals: Vec<Individual<R>>,
    individuals_are_sorted: bool,
    future: Vec<Individual<R>>,
}

impl<R: RunResult, Vm: VirtualMachine> Island<R, Vm> {
    pub(crate) fn new(callbacks: Box<dyn IslandCallbacks<R, Vm>>) -> Island<R, Vm> {
        Island { functions: callbacks, individuals: vec![], individuals_are_sorted: false, future: vec![] }
    }

    /// Resets the island to it's 'new' state.
    pub fn clear(&mut self) {
        self.individuals.clear();
        self.individuals_are_sorted = false;
        self.future.clear();
    }

    /// Returns the most fit of all the individuals (the one sorted to the tail by the sorting algorithm). Returns None
    /// if there are no Individuals or if the individuals have not been sorted
    pub fn most_fit_individual(&self) -> Option<&Individual<R>> {
        if !self.individuals_are_sorted {
            return None;
        }
        self.individuals.last()
    }

    /// Returns the least fit of all the individuals (the one sorted to the head by the sorting algorithm). Returns None
    /// if there are no Individuals or if the individuals have not been sorted
    pub fn least_fit_individual(&self) -> Option<&Individual<R>> {
        if !self.individuals_are_sorted {
            return None;
        }
        self.individuals.first()
    }

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
    pub fn sort_individuals(&mut self) {
        // It is useful to swap the Vec into a local variable to avoid borrow-checking issues during the sort
        let mut local_individuals = vec![];
        std::mem::swap(&mut self.individuals, &mut local_individuals);
        local_individuals.sort_by(|a, b| self.functions.sort_individuals(a, b));
        std::mem::swap(&mut self.individuals, &mut local_individuals);
        self.individuals_are_sorted = true;
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
        self.individuals_are_sorted = false;
        std::mem::swap(&mut self.individuals, &mut self.future);
    }

    /// Select one individual from the island according to the specified SelectionCurve and borrow it.
    /// Returns the individual borrowed or None if the population is zero or not sorted
    pub fn select_one_individual<Rnd: rand::Rng>(
        &self,
        curve: SelectionCurve,
        rng: &mut Rnd,
    ) -> Option<&Individual<R>> {
        if !self.individuals_are_sorted {
            return None;
        }

        let max = self.individuals.len();
        if max == 0 {
            None
        } else {
            self.individuals.get(curve.pick_one_index(rng, max))
        }
    }

    /// Select one individual from the island according to the specified SelectionCurve and remove it permanently.
    /// Returns the individual removed or None if the population is zero or not sorted
    pub fn select_and_remove_one_individual<Rnd: rand::Rng>(
        &mut self,
        curve: SelectionCurve,
        rng: &mut Rnd,
    ) -> Option<Individual<R>> {
        if !self.individuals_are_sorted {
            return None;
        }

        let max = self.individuals.len();
        if max == 0 {
            None
        } else {
            Some(self.individuals.remove(curve.pick_one_index(rng, max)))
        }
    }

    /// Adds an individual to the future generation
    pub fn add_individual_to_future_generation(&mut self, individual: Individual<R>) {
        self.future.push(individual);
    }
}

impl<R: RunResult, Vm: VirtualMachine> std::fmt::Debug for Box<dyn IslandCallbacks<R, Vm>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.as_ref())
    }
}

impl<R: RunResult, Vm: VirtualMachine> PartialEq for Island<R, Vm> {
    fn eq(&self, other: &Self) -> bool {
        self.functions.as_ref() as *const _ == other.functions.as_ref() as *const _ && 
        self.individuals == other.individuals &&
        self.individuals_are_sorted == other.individuals_are_sorted &&
        self.future == other.future
    }
}

