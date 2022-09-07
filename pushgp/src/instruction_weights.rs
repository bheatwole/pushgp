use crate::{Code, Configuration, StaticInstruction, VirtualMachine, VirtualMachineEngine};

pub type GenerateFn<Vm> = fn(engine: &mut VirtualMachineEngine<Vm>) -> Code<Vm>;

/// This struct tracks the weights associated with each instruction, and allows quickly picking a random instruction.
#[derive(PartialEq)]
pub struct InstructionWeights<Vm: VirtualMachine> {
    instructions: Vec<InstructionEntry<Vm>>,
    sum_of_weights: usize,
}

impl<Vm: VirtualMachine> InstructionWeights<Vm> {
    pub fn new() -> InstructionWeights<Vm> {
        InstructionWeights { instructions: vec![], sum_of_weights: 0 }
    }

    /// Adds the specified instruction to the weight list. The instruction must meet all the compile-time traits of the
    /// Vm or this will fail to compile. (i.e. if the Vm does not implement VirtualMachineMustHaveName, adding an
    /// instruction that uses the Name stack will fail to compile.)
    pub fn add_instruction<C: StaticInstruction<Vm>>(&mut self, weight: u8) {
        self.sum_of_weights += weight as usize;
        self.instructions.push(InstructionEntry {
            name: C::static_name(),
            weight: weight,
            combined_weight: self.sum_of_weights,
            generate: C::random_value,
        });
    }

    /// Returns the name of every instruction added to the weight table
    pub fn get_instruction_names(&self) -> Vec<&'static str> {
        let mut names = vec![];
        for entry in self.instructions.iter() {
            names.push(entry.name);
        }
        names
    }

    /// Returns the weight of the instruction with the specified name, or None
    pub fn weight_of_named_instruction(&self, name: &'static str) -> Option<u8> {
        if let Some(entry) = self.instructions.iter().find(|entry| entry.name == name) {
            Some(entry.weight)
        } else {
            None
        }
    }

    /// One of the possible genetic algorithms is to adjust the weights of instructions of new random indiviudals such
    /// that we get the best possible outcome from random code by limiting instructions that don't help and increasing
    /// the liklihood of instructions that do.
    ///
    /// This function resets the weights of all instructions based on a new configuration.
    pub fn reset_weights_from_configuration(&mut self, config: &Configuration) {
        let mut next_sum_of_weights = 0;
        for entry in self.instructions.iter_mut() {
            next_sum_of_weights += config.get_instruction_weight(entry.name) as usize;
            entry.combined_weight = next_sum_of_weights;
        }
        self.sum_of_weights = next_sum_of_weights;
    }

    /// Returns the total sum of all weights so that other code can include a range of all instruction weights in a
    /// grand total random number choice.
    pub fn get_sum_of_weights(&self) -> usize {
        self.sum_of_weights
    }

    /// Picks a random instruction from all the instructions that are defined, and returns a function that will create
    /// a new piece of Code when called.
    pub fn pick_random_instruction_generator<R: rand::Rng>(&self, rng: &mut R) -> GenerateFn<Vm> {
        let pick = rng.gen_range(1..=self.sum_of_weights);
        let index = self.instructions.partition_point(|entry| entry.combined_weight < pick);
        self.instructions.get(index).unwrap().generate
    }
}

// The default implementation is too chatty for this object, which appears in the test output and obfuscates the actual
// test results.
impl<Vm: VirtualMachine> std::fmt::Debug for InstructionWeights<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionWeights: weight of {} for {} instructions", self.sum_of_weights, self.instructions.len())
    }
}

struct InstructionEntry<Vm: VirtualMachine> {
    pub name: &'static str,

    // The weight assigned to this instruction
    pub weight: u8,

    // The weight of this entry combined with the sum of weight of every entry sorted before it.
    pub combined_weight: usize,

    // The function to call to get a new random value of this instruction
    pub generate: GenerateFn<Vm>,
}

impl<Vm: VirtualMachine> std::fmt::Debug for InstructionEntry<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionEntry for {}", self.name)
    }
}

impl<Vm: VirtualMachine> std::cmp::PartialEq for InstructionEntry<Vm> {
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
