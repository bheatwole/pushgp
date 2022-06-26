use crate::{Code, Configuration, StaticInstruction, VirtualMachine};

pub type GenerateFn<Vm> = fn(vm: &mut Vm) -> Code<Vm>;

#[derive(PartialEq)]
pub struct InstructionWeights<Vm: VirtualMachine> {
    instructions: Vec<InstructionEntry<Vm>>,
    sum_of_weights: usize,
}

impl<Vm: VirtualMachine> InstructionWeights<Vm> {
    pub fn new() -> InstructionWeights<Vm> {
        InstructionWeights { instructions: vec![], sum_of_weights: 0 }
    }

    pub fn add_instruction<C: StaticInstruction<Vm>>(&mut self, weight: u8) {
        self.sum_of_weights += weight as usize;
        self.instructions.push(InstructionEntry {
            name: C::static_name(),
            combined_weight: self.sum_of_weights,
            generate: C::random_value,
        });
    }

    pub fn reset_weights_from_configuration(&mut self, config: &Configuration) {
        let mut next_sum_of_weights = 0;
        for entry in self.instructions.iter_mut() {
            next_sum_of_weights += config.get_instruction_weight(entry.name) as usize;
            entry.combined_weight = next_sum_of_weights;
        }
        self.sum_of_weights = next_sum_of_weights;
    }

    pub fn get_sum_of_weights(&self) -> usize {
        self.sum_of_weights
    }

    pub fn pick_random_instruction_generator<R: rand::Rng>(&self, rng: &mut R) -> GenerateFn<Vm> {
        let pick = rng.gen_range(1..=self.sum_of_weights);
        let index = self.instructions.partition_point(|entry| entry.combined_weight < pick);
        self.instructions.get(index).unwrap().generate
    }
}

// The default implementation is too chatty for this object, which appears in the test output
impl<Vm: VirtualMachine> std::fmt::Debug for InstructionWeights<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionWeights: weight of {} for {} instructions", self.sum_of_weights, self.instructions.len())
    }
}

struct InstructionEntry<Vm> {
    pub name: &'static str,

    // The weight of this entry combined with the sum of weight of every entry sorted before it.
    pub combined_weight: usize,

    // The function to call to get a new random value of this instruction
    pub generate: GenerateFn<Vm>,
}

impl<Vm> std::fmt::Debug for InstructionEntry<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionEntry for {}", self.name)
    }
}

impl<Vm> std::cmp::PartialEq for InstructionEntry<Vm> {
    fn eq(&self, other: &InstructionEntry<Vm>) -> bool {
        if self.name == other.name && self.combined_weight == other.combined_weight {
            let lhs = self.generate as usize;
            let rhs = self.generate as usize;
            lhs == rhs
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn verify_partition_point_function() {
        // The instruction entries table depend upon the following behavior from partition_point. If it ever stops
        // working like this, we need to know. Specifically only the first of a series of identical values is returned
        let entries = [1, 5, 5, 5, 10];
        assert_eq!(0, entries.partition_point(|&x| x < 1));
        assert_eq!(1, entries.partition_point(|&x| x < 2));
        assert_eq!(1, entries.partition_point(|&x| x < 3));
        assert_eq!(1, entries.partition_point(|&x| x < 4));
        assert_eq!(1, entries.partition_point(|&x| x < 5));
        assert_eq!(4, entries.partition_point(|&x| x < 6));
        assert_eq!(4, entries.partition_point(|&x| x < 10));
    }
}