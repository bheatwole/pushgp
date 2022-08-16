use fnv::FnvHashMap;
use rand::{rngs::SmallRng, SeedableRng};

use crate::*;

#[derive(Debug, PartialEq)]
pub struct VirtualMachineEngine<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> {
    rng: SmallRng,
    exec_stack: Stack<Exec<Vm>>,
    parser: Parser<Vm>,
    config: Configuration,
    weights: InstructionWeights<Vm>,
    defined_names: FnvHashMap<Name, Code<Vm>>,
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> VirtualMachineEngine<Vm> {
    pub fn new(seed: Option<u64>, config: Configuration) -> VirtualMachineEngine<Vm> {
        let vm = VirtualMachineEngine {
            rng: small_rng_from_optional_seed(seed),
            exec_stack: Stack::new(),
            parser: Parser::new(),
            config,
            weights: InstructionWeights::new(),
            defined_names: FnvHashMap::default(),
        };

        vm
    }

    pub fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }

    pub fn set_rng_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    pub fn exec(&mut self) -> &mut Stack<Code<Vm>> {
        &mut self.exec_stack
    }

    pub fn get_weights(&self) -> &InstructionWeights<Vm> {
        &self.weights
    }

    pub fn clear(&mut self) {
        self.exec_stack.clear();
        self.defined_names.clear();
    }

    pub fn add_instruction<C: StaticInstruction<Vm>>(&mut self) {
        self.parser.add_instruction::<C>();
        self.weights.add_instruction::<C>(self.config.get_instruction_weight(C::static_name()));
    }

    pub fn get_configuration(&self) -> &Configuration {
        &self.config
    }

    pub fn reset_configuration(&mut self, config: Configuration) {
        self.config = config;

        // Iterate through all instruction names and re-assign the weights for the instructions
        self.weights.reset_weights_from_configuration(&self.config);
    }

    pub fn get_instruction_weights(&self) -> &InstructionWeights<Vm> {
        &self.weights
    }

    pub fn pick_random_instruction_generator(&mut self) -> GenerateFn<Vm> {
        self.weights.pick_random_instruction_generator(&mut self.rng)
    }

    pub fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Vm>> {
        self.parser.parse(input)
    }

    pub fn must_parse<'a>(&self, input: &'a str) -> Code<Vm> {
        let (rest, code) = self.parse(input).unwrap();
        assert_eq!(rest.len(), 0);
        code
    }

    pub fn parse_and_set_code(&mut self, input: &str) -> Result<(), ParseError> {
        self.clear();
        let (rest, code) = self.parse(input).map_err(|e| ParseError::new(e))?;
        if rest.len() == 0 {
            self.exec_stack.push(code);
            Ok(())
        } else {
            return Err(ParseError::new_with_message("the code did not finish parsing"));
        }
    }

    pub fn set_code(&mut self, code: Code<Vm>) {
        self.clear();
        self.exec_stack.push(code);
    }
}

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}