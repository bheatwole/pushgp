use crate::*;
use rand::prelude::SeedableRng;
use rand::rngs::SmallRng;

pub trait VirtualMachine: Sized {
    /// All virtual machines must expose a random number generator.
    fn get_rng(&mut self) -> &mut SmallRng;

    /// Various algorithms need to reliably repeat random number generation.
    fn set_rng_seed(&mut self, seed: Option<u64>);

    /// Clears all the stacks and defined names
    fn clear(&mut self);

    fn run(&mut self, max: usize) -> usize;
    fn next(&mut self) -> Option<usize>;

    fn add_instruction<C: StaticInstruction<Self>>(&mut self);
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Self>>;
    fn must_parse(&self, input: &str) -> Code<Self> {
        let (rest, code) = self.parse(input).unwrap();
        assert_eq!(rest.len(), 0);
        code
    }

    fn parse_and_set_code(&mut self, input: &str) -> Result<(), ParseError>;
    fn set_code(&mut self, code: Code<Self>);
}

#[derive(Debug, PartialEq)]
pub struct BaseVm {
    rng: SmallRng,
    exec_stack: Stack<Exec<BaseVm>>,
    bool_stack: Stack<Bool>,
    code_stack: Stack<Code<BaseVm>>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack<BaseVm>,
    parser: Parser<BaseVm>,
}

impl BaseVm {
    pub fn new() -> BaseVm {
        let vm = BaseVm {
            rng: small_rng_from_optional_seed(None),
            exec_stack: Stack::new(),
            bool_stack: Stack::new(),
            code_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: NameStack::new(),
            parser: Parser::new(),
        };

        vm
    }
}

impl VirtualMachine for BaseVm {
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }

    fn set_rng_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    fn clear(&mut self) {
        self.exec_stack.clear();
        self.bool_stack.clear();
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
        self.parser.add_instruction::<C>()
        // TODO: Add to random weight table
    }

    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<BaseVm>> {
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
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

impl VirtualMachineMustHaveBool<BaseVm> for BaseVm {
    fn bool(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack
    }
}

impl VirtualMachineMustHaveCode<BaseVm> for BaseVm {
    fn code(&mut self) -> &mut Stack<Code<BaseVm>> {
        &mut self.code_stack
    }
}

impl VirtualMachineMustHaveExec<BaseVm> for BaseVm {
    fn exec(&mut self) -> &mut Stack<Code<BaseVm>> {
        &mut self.exec_stack
    }
}

impl VirtualMachineMustHaveFloat<BaseVm> for BaseVm {
    fn float(&mut self) -> &mut Stack<Float> {
        &mut self.float_stack
    }
}

impl VirtualMachineMustHaveInteger<BaseVm> for BaseVm {
    fn integer(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
    }
}

impl VirtualMachineMustHaveName<BaseVm> for BaseVm {
    fn name(&mut self) -> &mut NameStack<BaseVm> {
        &mut self.name_stack
    }
}
