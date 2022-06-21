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

    fn set_code(&mut self, input: &str) -> Result<(), ParseError>;
}

#[derive(Debug, PartialEq)]
pub struct BaseVm {
    rng: SmallRng,
    exec_stack: Stack<Exec<BaseVm>>,
    bool_stack: Stack<Bool>,
    code_stack: Stack<Code<BaseVm>>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: Stack<String>,
    parser: Parser<BaseVm>,
    quote_next_name: bool,
    defined_names: fnv::FnvHashMap<String, Code<BaseVm>>,
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
            name_stack: Stack::new(),
            parser: Parser::new(),
            quote_next_name: false,
            defined_names: fnv::FnvHashMap::default(),
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
        self.quote_next_name = false;
        self.defined_names.clear();
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

    fn set_code(&mut self, input: &str) -> Result<(), ParseError> {
        self.clear();
        let (rest, code) = self.parse(input).map_err(|e| ParseError::new(e))?;
        if rest.len() == 0 {
            self.exec_stack.push(code);
            Ok(())
        } else {
            return Err(ParseError::new_with_message("the code did not finish parsing"));
        }
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
    fn name(&mut self) -> &mut Stack<String> {
        &mut self.name_stack
    }

    /// Returns true if the next Name encountered on the Exec stack should be pushed to the Name stack instead of
    /// possibly running the Code associated with the Name.
    fn should_quote_next_name(&self) -> bool {
        self.quote_next_name
    }

    /// Sets whether or not the next Name encountered on the Exec stack should be pushed to the Name stack instead of
    /// possibly running the Code associated with the Name. Uses interior mutability.
    fn set_should_quote_next_name(&mut self, quote_next_name: bool) {
        self.quote_next_name = quote_next_name;
    }

    /// Returns the Code defined with the specified Name or None
    fn definition_for_name(&self, name: &String) -> Option<Box<dyn Instruction<BaseVm>>> {
        self.defined_names.get(name).map(|c| c.clone())
    }

    /// Defines the Code that will be placed on the top of the Exec stack when the specified Name is encountered. If the
    /// name was previously defined, the new definition replaces the old value.
    fn define_name(&mut self, name: String, code: Box<dyn Instruction<BaseVm>>) {
        self.defined_names.insert(name, code);
    }

    /// Returns a list of all previously defined names. May be empty if no names have been defined
    fn all_defined_names(&self) -> Vec<String> {
        self.defined_names.keys().map(|k| k.clone()).collect()
    }

    /// Returns the number of previously defined names. Avoids an expensive copy of all names if only the count is
    /// needed.
    fn defined_names_len(&self) -> usize {
        self.defined_names.len()
    }
}
