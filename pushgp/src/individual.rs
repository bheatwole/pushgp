use crate::Instruction;

pub struct Individual<S, Vm> {
    code: Box<dyn Instruction<Vm>>,
    state: S,
}

impl<S, Vm> Individual<S, Vm> {
    pub fn new(code: Box<dyn Instruction<Vm>>, initial_state: S) -> Individual<S, Vm> {
        Individual { code, state: initial_state }
    }

    pub fn get_code(&self) -> Box<dyn Instruction<Vm>> {
        self.code.clone()
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.state
    }
}
