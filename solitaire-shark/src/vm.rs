use crate::{Card, GameState, VirtualMachineMustHaveCard};
use pushgp::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SolitareVm {
    engine: VirtualMachineEngine<SolitareVm>,
    bool_stack: Stack<Bool>,
    card_stack: Stack<Card>,
    code_stack: Stack<Code>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack,
    game: GameState,
}

impl SolitareVm {
    pub fn new(seed: u64, config: Configuration) -> SolitareVm {
        let vm = SolitareVm {
            engine: VirtualMachineEngine::new(Some(seed), config, 40),
            bool_stack: Stack::new(200),
            card_stack: Stack::new(200),
            code_stack: Stack::new(20),
            integer_stack: Stack::new(200),
            name_stack: NameStack::new(200),
            game: GameState::new(seed),
        };

        vm
    }

    pub fn swap_game_state(&mut self, mut to_swap: GameState) -> GameState {
        std::mem::swap(&mut self.game, &mut to_swap);
        to_swap
    }
}

impl VirtualMachine for SolitareVm {
    fn engine(&self) -> &VirtualMachineEngine<Self> {
        &self.engine
    }

    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self> {
        &mut self.engine
    }

    fn clear(&mut self) {
        self.engine.clear();
        self.bool_stack.clear();
        self.card_stack.clear();
        self.code_stack.clear();
        self.integer_stack.clear();
        self.name_stack.clear();
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
    fn code(&mut self) -> &mut Stack<Code> {
        &mut self.code_stack
    }
}

impl VirtualMachineMustHaveExec<SolitareVm> for SolitareVm {
    fn exec(&mut self) -> &mut Stack<Code> {
        self.engine.exec()
    }
}

impl VirtualMachineMustHaveInteger<SolitareVm> for SolitareVm {
    fn integer(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
    }
}

impl VirtualMachineMustHaveName<SolitareVm> for SolitareVm {
    fn name(&mut self) -> &mut NameStack {
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

impl OpcodeConvertor for SolitareVm {
    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.engine().name_for_opcode(opcode)
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.engine().opcode_for_name(name)
    }
}

pub fn add_instructions(vm: &mut SolitareVm) {
    vm.engine_mut().add_instruction::<pushgp::BoolAnd>();
    vm.engine_mut().add_instruction::<pushgp::BoolDefine>();
    vm.engine_mut().add_instruction::<pushgp::BoolDup>();
    vm.engine_mut().add_instruction::<pushgp::BoolEqual>();
    vm.engine_mut().add_instruction::<pushgp::BoolFlush>();
    vm.engine_mut().add_instruction::<pushgp::BoolFromInt>();
    vm.engine_mut().add_instruction::<pushgp::BoolNot>();
    vm.engine_mut().add_instruction::<pushgp::BoolOr>();
    vm.engine_mut().add_instruction::<pushgp::BoolPop>();
    vm.engine_mut().add_instruction::<pushgp::BoolRand>();
    vm.engine_mut().add_instruction::<pushgp::BoolRot>();
    vm.engine_mut().add_instruction::<pushgp::BoolShove>();
    vm.engine_mut().add_instruction::<pushgp::BoolStackDepth>();
    vm.engine_mut().add_instruction::<pushgp::BoolSwap>();
    vm.engine_mut().add_instruction::<pushgp::BoolYankDup>();
    vm.engine_mut().add_instruction::<pushgp::BoolYank>();
    vm.engine_mut().add_instruction::<pushgp::CodeAppend>();
    vm.engine_mut().add_instruction::<pushgp::CodeAtom>();
    vm.engine_mut().add_instruction::<pushgp::CodeCar>();
    vm.engine_mut().add_instruction::<pushgp::CodeCdr>();
    vm.engine_mut().add_instruction::<pushgp::CodeCons>();
    vm.engine_mut().add_instruction::<pushgp::CodeContainer>();
    vm.engine_mut().add_instruction::<pushgp::CodeContains>();
    vm.engine_mut().add_instruction::<pushgp::CodeDefine>();
    vm.engine_mut().add_instruction::<pushgp::CodeDefinition>();
    vm.engine_mut().add_instruction::<pushgp::CodeDiscrepancy>();
    vm.engine_mut().add_instruction::<pushgp::CodeDoNCount>();
    vm.engine_mut().add_instruction::<pushgp::CodeDoNRange>();
    vm.engine_mut().add_instruction::<pushgp::CodeDoNTimes>();
    vm.engine_mut().add_instruction::<pushgp::CodeDoN>();
    vm.engine_mut().add_instruction::<pushgp::CodeDo>();
    vm.engine_mut().add_instruction::<pushgp::CodeDup>();
    vm.engine_mut().add_instruction::<pushgp::CodeEqual>();
    vm.engine_mut().add_instruction::<pushgp::CodeExtract>();
    vm.engine_mut().add_instruction::<pushgp::CodeFlush>();
    vm.engine_mut().add_instruction::<pushgp::CodeFromBoolean>();
    vm.engine_mut().add_instruction::<pushgp::CodeFromInteger>();
    vm.engine_mut().add_instruction::<pushgp::CodeFromName>();
    vm.engine_mut().add_instruction::<pushgp::CodeIf>();
    vm.engine_mut().add_instruction::<pushgp::CodeInsert>();
    vm.engine_mut().add_instruction::<pushgp::CodeLength>();
    vm.engine_mut().add_instruction::<pushgp::CodeList>();
    vm.engine_mut().add_instruction::<pushgp::CodeMember>();
    vm.engine_mut().add_instruction::<pushgp::CodeNoop>();
    vm.engine_mut().add_instruction::<pushgp::CodeNthCdr>();
    vm.engine_mut().add_instruction::<pushgp::CodeNth>();
    vm.engine_mut().add_instruction::<pushgp::CodeNull>();
    vm.engine_mut().add_instruction::<pushgp::CodePop>();
    vm.engine_mut().add_instruction::<pushgp::CodePosition>();
    vm.engine_mut().add_instruction::<pushgp::CodeQuote>();
    vm.engine_mut().add_instruction::<pushgp::CodeRand>();
    vm.engine_mut().add_instruction::<pushgp::CodeRot>();
    vm.engine_mut().add_instruction::<pushgp::CodeShove>();
    vm.engine_mut().add_instruction::<pushgp::CodeSize>();
    vm.engine_mut().add_instruction::<pushgp::CodeStackDepth>();
    vm.engine_mut().add_instruction::<pushgp::CodeSubstitute>();
    vm.engine_mut().add_instruction::<pushgp::CodeSwap>();
    vm.engine_mut().add_instruction::<pushgp::CodeYankDup>();
    vm.engine_mut().add_instruction::<pushgp::CodeYank>();
    vm.engine_mut().add_instruction::<pushgp::ExecDefine>();
    vm.engine_mut().add_instruction::<pushgp::ExecDoNCount>();
    vm.engine_mut().add_instruction::<pushgp::ExecDoNRange>();
    vm.engine_mut().add_instruction::<pushgp::ExecDoNTimes>();
    vm.engine_mut().add_instruction::<pushgp::ExecDup>();
    vm.engine_mut().add_instruction::<pushgp::ExecEqual>();
    vm.engine_mut().add_instruction::<pushgp::ExecFlush>();
    vm.engine_mut().add_instruction::<pushgp::ExecIf>();
    vm.engine_mut().add_instruction::<pushgp::ExecK>();
    vm.engine_mut().add_instruction::<pushgp::ExecPop>();
    vm.engine_mut().add_instruction::<pushgp::ExecRot>();
    vm.engine_mut().add_instruction::<pushgp::ExecShove>();
    vm.engine_mut().add_instruction::<pushgp::ExecStackDepth>();
    vm.engine_mut().add_instruction::<pushgp::ExecSwap>();
    vm.engine_mut().add_instruction::<pushgp::ExecS>();
    vm.engine_mut().add_instruction::<pushgp::ExecYankDup>();
    vm.engine_mut().add_instruction::<pushgp::ExecYank>();
    vm.engine_mut().add_instruction::<pushgp::ExecY>();
    vm.engine_mut().add_instruction::<pushgp::IntegerDefine>();
    vm.engine_mut()
        .add_instruction::<pushgp::IntegerDifference>();
    vm.engine_mut().add_instruction::<pushgp::IntegerDup>();
    vm.engine_mut().add_instruction::<pushgp::IntegerEqual>();
    vm.engine_mut().add_instruction::<pushgp::IntegerFlush>();
    vm.engine_mut()
        .add_instruction::<pushgp::IntegerFromBoolean>();
    vm.engine_mut().add_instruction::<pushgp::IntegerGreater>();
    vm.engine_mut().add_instruction::<pushgp::IntegerLess>();
    vm.engine_mut().add_instruction::<pushgp::IntegerMax>();
    vm.engine_mut().add_instruction::<pushgp::IntegerMin>();
    vm.engine_mut().add_instruction::<pushgp::IntegerModulo>();
    vm.engine_mut().add_instruction::<pushgp::IntegerPop>();
    vm.engine_mut().add_instruction::<pushgp::IntegerProduct>();
    vm.engine_mut().add_instruction::<pushgp::IntegerQuotient>();
    vm.engine_mut().add_instruction::<pushgp::IntegerRand>();
    vm.engine_mut().add_instruction::<pushgp::IntegerRot>();
    vm.engine_mut().add_instruction::<pushgp::IntegerShove>();
    vm.engine_mut()
        .add_instruction::<pushgp::IntegerStackDepth>();
    vm.engine_mut().add_instruction::<pushgp::IntegerSum>();
    vm.engine_mut().add_instruction::<pushgp::IntegerSwap>();
    vm.engine_mut().add_instruction::<pushgp::IntegerYankDup>();
    vm.engine_mut().add_instruction::<pushgp::IntegerYank>();
    vm.engine_mut().add_instruction::<pushgp::NameDup>();
    vm.engine_mut().add_instruction::<pushgp::NameEqual>();
    vm.engine_mut().add_instruction::<pushgp::NameFlush>();
    vm.engine_mut().add_instruction::<pushgp::NamePop>();
    vm.engine_mut().add_instruction::<pushgp::NameQuote>();
    vm.engine_mut()
        .add_instruction::<pushgp::NameRandBoundName>();
    vm.engine_mut().add_instruction::<pushgp::NameRand>();
    vm.engine_mut().add_instruction::<pushgp::NameRot>();
    vm.engine_mut().add_instruction::<pushgp::NameShove>();
    vm.engine_mut().add_instruction::<pushgp::NameStackDepth>();
    vm.engine_mut().add_instruction::<pushgp::NameSwap>();
    vm.engine_mut().add_instruction::<pushgp::NameYankDup>();
    vm.engine_mut().add_instruction::<pushgp::NameYank>();

    // Here are our instructions:
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardDrawNextThree>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardDrawPileLen>();
    vm.engine_mut().add_instruction::<crate::card::CardDup>();
    vm.engine_mut().add_instruction::<crate::card::CardEqual>();
    vm.engine_mut().add_instruction::<crate::card::CardFlush>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardFromInt>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardLiteralValue>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardMoveTopPlayPileCardToFinish>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardMoveTopWorkPileCardToFinish>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardMoveWorkPileCardsToAnotherWorkPile>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardPlayPileLen>();
    vm.engine_mut().add_instruction::<crate::card::CardPop>();
    vm.engine_mut().add_instruction::<crate::card::CardRand>();
    vm.engine_mut()
        .add_instruction::<crate::card::CardReadyToFinish>();
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();
    vm.engine_mut().add_instruction::<crate::card::CardDefine>();

    // These must be last, with Name the very last of all. The reason is that parsing runs in order from top to bottom
    // and all the 'normal' instructions use an exact match. However the literal values use more involved parsing and
    // Name is the catch-all (anything that does not parsed earlier will become a Name up to the next white-space).
    vm.engine_mut()
        .add_instruction::<pushgp::BoolLiteralValue>();
    vm.engine_mut()
        .add_instruction::<pushgp::IntegerLiteralValue>();
    vm.engine_mut()
        .add_instruction::<pushgp::NameLiteralValue>();
}
