use crate::Code;

pub struct Individual<S> {
    code: Code,
    state: S,
}

impl<S> Individual<S> {
    pub fn new(code: Code, initial_state: S) -> Individual<S> {
        Individual { code, state: initial_state }
    }

    pub fn get_code(&self) -> Code {
        self.code.clone()
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.state
    }
}
