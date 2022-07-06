use rand::rngs::SmallRng;
use pushgp::*;
use crate::{Card, VirtualMachineMustHaveCard};

#[derive(Debug, PartialEq)]
pub struct SolitareVm {
    rng: SmallRng,
    exec_stack: Stack<Exec<SolitareVm>>,
    bool_stack: Stack<Bool>,
    card_stack: Stack<Card>,
    code_stack: Stack<Code<SolitareVm>>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack<SolitareVm>,
    parser: Parser<SolitareVm>,
    config: Configuration,
    weights: InstructionWeights<SolitareVm>,
}

impl SolitareVm {
    pub fn new(seed: Option<u64>, config: Configuration) -> SolitareVm {
        let vm = SolitareVm {
            rng: small_rng_from_optional_seed(seed),
            exec_stack: Stack::new(),
            bool_stack: Stack::new(),
            card_stack: Stack::new(),
            code_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: NameStack::new(),
            parser: Parser::new(),
            config,
            weights: InstructionWeights::new(),
        };

        vm
    }
}

impl VirtualMachine for SolitareVm {
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }

    fn set_rng_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    fn clear(&mut self) {
        self.exec_stack.clear();
        self.bool_stack.clear();
        self.card_stack.clear();
        self.code_stack.clear();
        self.float_stack.clear();
        self.integer_stack.clear();
        self.name_stack.clear();
    }

    fn run(&mut self, max: usize) -> usize {
        // trace!("{:?}", self);
        let mut total_count = 0;
        while let Some(count) = self.next() {
            total_count += count;
            if total_count >= max {
                break;
            }
        }
        total_count
    }

    fn next(&mut self) -> Option<usize> {
        // Pop the top piece of code from the exec stack and execute it.
        if let Some(mut exec) = self.exec_stack.pop() {
            exec.execute(self);

            // Return the number of points required to perform that action
            return Some(1);
        }

        // No action was found
        None
    }

    fn add_instruction<C: StaticInstruction<Self>>(&mut self) {
        self.parser.add_instruction::<C>();
        self.weights.add_instruction::<C>(self.config.get_instruction_weight(C::static_name()));
    }

    fn get_configuration(&self) -> &Configuration {
        &self.config
    }

    fn reset_configuration(&mut self, config: Configuration) {
        self.config = config;

        // Iterate through all instruction names and re-assign the weights for the instructions
        self.weights.reset_weights_from_configuration(&self.config);
    }

    fn get_instruction_weights(&self) -> &InstructionWeights<Self> {
        &self.weights
    }

    fn generate_random_instruction(&mut self) -> Code<Self> {
        let generator = self.weights.pick_random_instruction_generator(&mut self.rng);
        generator(self)
    }

    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<SolitareVm>> {
        self.parser.parse(input)
    }

    fn parse_and_set_code(&mut self, input: &str) -> Result<(), ParseError> {
        self.clear();
        let (rest, code) = self.parse(input).map_err(|e| ParseError::new(e))?;
        if rest.len() == 0 {
            self.exec_stack.push(code);
            Ok(())
        } else {
            return Err(ParseError::new_with_message("the code did not finish parsing"));
        }
    }

    fn set_code(&mut self, code: Code<Self>) {
        self.clear();
        self.exec_stack.push(code);
    }
}

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    use rand::SeedableRng;

    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

impl VirtualMachineMustHaveBool<SolitareVm> for SolitareVm {
    fn bool(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack
    }
}

impl VirtualMachineMustHaveCard<SolitareVm> for SolitareVm {
    fn card(&mut self) -> &mut Stack<Card> {
        &mut self.card_stack
    }
}

impl VirtualMachineMustHaveCode<SolitareVm> for SolitareVm {
    fn code(&mut self) -> &mut Stack<Code<SolitareVm>> {
        &mut self.code_stack
    }
}

impl VirtualMachineMustHaveExec<SolitareVm> for SolitareVm {
    fn exec(&mut self) -> &mut Stack<Code<SolitareVm>> {
        &mut self.exec_stack
    }
}

impl VirtualMachineMustHaveFloat<SolitareVm> for SolitareVm {
    fn float(&mut self) -> &mut Stack<Float> {
        &mut self.float_stack
    }
}

impl VirtualMachineMustHaveInteger<SolitareVm> for SolitareVm {
    fn integer(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
    }
}

impl VirtualMachineMustHaveName<SolitareVm> for SolitareVm {
    fn name(&mut self) -> &mut NameStack<SolitareVm> {
        &mut self.name_stack
    }
}
