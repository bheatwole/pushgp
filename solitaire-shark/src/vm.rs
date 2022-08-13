use crate::{Card, GameState, VirtualMachineMustHaveCard};
use pushgp::*;
use rand::rngs::SmallRng;

#[derive(Debug, PartialEq)]
pub struct SolitareVm {
    rng: SmallRng,
    exec_stack: Stack<Exec<SolitareVm>>,
    bool_stack: Stack<Bool>,
    card_stack: Stack<Card>,
    code_stack: Stack<Code<SolitareVm>>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack<SolitareVm>,
    parser: Parser<SolitareVm>,
    config: Configuration,
    weights: InstructionWeights<SolitareVm>,
    game: GameState,
}

impl SolitareVm {
    pub fn new(seed: u64, config: Configuration) -> SolitareVm {
        let vm = SolitareVm {
            rng: small_rng_from_optional_seed(Some(seed)),
            exec_stack: Stack::new(),
            bool_stack: Stack::new(),
            card_stack: Stack::new(),
            code_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: NameStack::new(),
            parser: Parser::new(),
            config,
            weights: InstructionWeights::new(),
            game: GameState::new(seed),
        };

        vm
    }

    pub fn set_game_state(&mut self, mut new_state: GameState) {
        std::mem::swap(&mut self.game, &mut new_state)
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
        self.weights
            .add_instruction::<C>(self.config.get_instruction_weight(C::static_name()));
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
        let generator = self
            .weights
            .pick_random_instruction_generator(&mut self.rng);
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
            return Err(ParseError::new_with_message(
                "the code did not finish parsing",
            ));
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

impl DoesVirtualMachineHaveName for SolitareVm {
    const HAS_NAME: bool = true;
}

pub trait VirtualMachineMustHaveGame<Vm> {
    fn game(&mut self) -> &mut GameState;
}

impl VirtualMachineMustHaveGame<SolitareVm> for SolitareVm {
    fn game(&mut self) -> &mut GameState {
        &mut self.game
    }
}


pub fn add_instructions(
    vm: &mut SolitareVm,
) {
    vm.add_instruction::<pushgp::BoolAnd>();
    vm.add_instruction::<pushgp::BoolDefine>();
    vm.add_instruction::<pushgp::BoolDup>();
    vm.add_instruction::<pushgp::BoolEqual>();
    vm.add_instruction::<pushgp::BoolFlush>();
    vm.add_instruction::<pushgp::BoolFromInt>();
    vm.add_instruction::<pushgp::BoolNot>();
    vm.add_instruction::<pushgp::BoolOr>();
    vm.add_instruction::<pushgp::BoolPop>();
    vm.add_instruction::<pushgp::BoolRand>();
    vm.add_instruction::<pushgp::BoolRot>();
    vm.add_instruction::<pushgp::BoolShove>();
    vm.add_instruction::<pushgp::BoolStackDepth>();
    vm.add_instruction::<pushgp::BoolSwap>();
    vm.add_instruction::<pushgp::BoolYankDup>();
    vm.add_instruction::<pushgp::BoolYank>();
    vm.add_instruction::<pushgp::CodeAppend>();
    vm.add_instruction::<pushgp::CodeAtom>();
    vm.add_instruction::<pushgp::CodeCar>();
    vm.add_instruction::<pushgp::CodeCdr>();
    vm.add_instruction::<pushgp::CodeCons>();
    vm.add_instruction::<pushgp::CodeContainer>();
    vm.add_instruction::<pushgp::CodeContains>();
    vm.add_instruction::<pushgp::CodeCrossover>();
    vm.add_instruction::<pushgp::CodeDefine>();
    vm.add_instruction::<pushgp::CodeDefinition>();
    vm.add_instruction::<pushgp::CodeDiscrepancy>();
    vm.add_instruction::<pushgp::CodeDoNCount>();
    vm.add_instruction::<pushgp::CodeDoNRange>();
    vm.add_instruction::<pushgp::CodeDoNTimes>();
    vm.add_instruction::<pushgp::CodeDoN>();
    vm.add_instruction::<pushgp::CodeDo>();
    vm.add_instruction::<pushgp::CodeDup>();
    vm.add_instruction::<pushgp::CodeEqual>();
    vm.add_instruction::<pushgp::CodeExtract>();
    vm.add_instruction::<pushgp::CodeFlush>();
    vm.add_instruction::<pushgp::CodeFromBoolean>();
    vm.add_instruction::<pushgp::CodeFromInteger>();
    vm.add_instruction::<pushgp::CodeFromName>();
    vm.add_instruction::<pushgp::CodeIf>();
    vm.add_instruction::<pushgp::CodeInsert>();
    vm.add_instruction::<pushgp::CodeLength>();
    vm.add_instruction::<pushgp::CodeList>();
    vm.add_instruction::<pushgp::CodeMember>();
    vm.add_instruction::<pushgp::CodeMutate>();
    vm.add_instruction::<pushgp::CodeMutateNoName>();
    vm.add_instruction::<pushgp::CodeNoop>();
    vm.add_instruction::<pushgp::CodeNthCdr>();
    vm.add_instruction::<pushgp::CodeNth>();
    vm.add_instruction::<pushgp::CodeNull>();
    vm.add_instruction::<pushgp::CodePop>();
    vm.add_instruction::<pushgp::CodePosition>();
    vm.add_instruction::<pushgp::CodeQuote>();
    vm.add_instruction::<pushgp::CodeRand>();
    vm.add_instruction::<pushgp::CodeRandChild>();
    vm.add_instruction::<pushgp::CodeRandChildNoName>();
    vm.add_instruction::<pushgp::CodeRandNoName>();
    vm.add_instruction::<pushgp::CodeRot>();
    vm.add_instruction::<pushgp::CodeSelectGeneticOperation>();
    vm.add_instruction::<pushgp::CodeShove>();
    vm.add_instruction::<pushgp::CodeSize>();
    vm.add_instruction::<pushgp::CodeStackDepth>();
    vm.add_instruction::<pushgp::CodeSubstitute>();
    vm.add_instruction::<pushgp::CodeSwap>();
    vm.add_instruction::<pushgp::CodeYankDup>();
    vm.add_instruction::<pushgp::CodeYank>();
    vm.add_instruction::<pushgp::ExecDefine>();
    vm.add_instruction::<pushgp::ExecDoNCount>();
    vm.add_instruction::<pushgp::ExecDoNRange>();
    vm.add_instruction::<pushgp::ExecDoNTimes>();
    vm.add_instruction::<pushgp::ExecDup>();
    vm.add_instruction::<pushgp::ExecEqual>();
    vm.add_instruction::<pushgp::ExecFlush>();
    vm.add_instruction::<pushgp::ExecIf>();
    vm.add_instruction::<pushgp::ExecK>();
    vm.add_instruction::<pushgp::ExecPop>();
    vm.add_instruction::<pushgp::ExecRot>();
    vm.add_instruction::<pushgp::ExecShove>();
    vm.add_instruction::<pushgp::ExecStackDepth>();
    vm.add_instruction::<pushgp::ExecSwap>();
    vm.add_instruction::<pushgp::ExecS>();
    vm.add_instruction::<pushgp::ExecYankDup>();
    vm.add_instruction::<pushgp::ExecYank>();
    vm.add_instruction::<pushgp::ExecY>();
    vm.add_instruction::<pushgp::IntegerDefine>();
    vm.add_instruction::<pushgp::IntegerDifference>();
    vm.add_instruction::<pushgp::IntegerDup>();
    vm.add_instruction::<pushgp::IntegerEqual>();
    vm.add_instruction::<pushgp::IntegerFlush>();
    vm.add_instruction::<pushgp::IntegerFromBoolean>();
    vm.add_instruction::<pushgp::IntegerGreater>();
    vm.add_instruction::<pushgp::IntegerLess>();
    vm.add_instruction::<pushgp::IntegerMax>();
    vm.add_instruction::<pushgp::IntegerMin>();
    vm.add_instruction::<pushgp::IntegerModulo>();
    vm.add_instruction::<pushgp::IntegerPop>();
    vm.add_instruction::<pushgp::IntegerProduct>();
    vm.add_instruction::<pushgp::IntegerQuotient>();
    vm.add_instruction::<pushgp::IntegerRand>();
    vm.add_instruction::<pushgp::IntegerRot>();
    vm.add_instruction::<pushgp::IntegerShove>();
    vm.add_instruction::<pushgp::IntegerStackDepth>();
    vm.add_instruction::<pushgp::IntegerSum>();
    vm.add_instruction::<pushgp::IntegerSwap>();
    vm.add_instruction::<pushgp::IntegerYankDup>();
    vm.add_instruction::<pushgp::IntegerYank>();
    vm.add_instruction::<pushgp::NameDup>();
    vm.add_instruction::<pushgp::NameEqual>();
    vm.add_instruction::<pushgp::NameFlush>();
    vm.add_instruction::<pushgp::NamePop>();
    vm.add_instruction::<pushgp::NameQuote>();
    vm.add_instruction::<pushgp::NameRandBoundName>();
    vm.add_instruction::<pushgp::NameRand>();
    vm.add_instruction::<pushgp::NameRot>();
    vm.add_instruction::<pushgp::NameShove>();
    vm.add_instruction::<pushgp::NameStackDepth>();
    vm.add_instruction::<pushgp::NameSwap>();
    vm.add_instruction::<pushgp::NameYankDup>();
    vm.add_instruction::<pushgp::NameYank>();
    
    // Here are our instructions:
    vm.add_instruction::<crate::card::CardDefine>();
    vm.add_instruction::<crate::card::CardDrawNextThree>();
    vm.add_instruction::<crate::card::CardDrawPileLen>();
    vm.add_instruction::<crate::card::CardDup>();
    vm.add_instruction::<crate::card::CardEqual>();
    vm.add_instruction::<crate::card::CardFlush>();
    vm.add_instruction::<crate::card::CardFromInt>();
    vm.add_instruction::<crate::card::CardLiteralValue>();
    vm.add_instruction::<crate::card::CardMoveTopPlayPileCardToFinish>();
    vm.add_instruction::<crate::card::CardMoveTopWorkPileCardToFinish>();
    vm.add_instruction::<crate::card::CardMoveWorkPileCardsToAnotherWorkPile>();
    vm.add_instruction::<crate::card::CardPlayPileLen>();
    vm.add_instruction::<crate::card::CardPop>();
    vm.add_instruction::<crate::card::CardRand>();
    vm.add_instruction::<crate::card::CardReadyToFinish>();
    vm.add_instruction::<crate::card::CardDefine>();
    vm.add_instruction::<crate::card::CardDefine>();
    vm.add_instruction::<crate::card::CardDefine>();
    vm.add_instruction::<crate::card::CardDefine>();
    vm.add_instruction::<crate::card::CardDefine>();

    // These must be last, with Name the very last of all. The reason is that parsing runs in order from top to bottom
    // and all the 'normal' instructions use an exact match. However the literal values use more involved parsing and
    // Name is the catch-all (anything that does not parse earlier will become a Name up to the next white-space).
    vm.add_instruction::<pushgp::PushList<SolitareVm>>();
    vm.add_instruction::<pushgp::BoolLiteralValue>();
    vm.add_instruction::<pushgp::IntegerLiteralValue>();
    vm.add_instruction::<pushgp::NameLiteralValue>();
}