use crate::{Code, Literal};

pub struct Individual<S, L: Literal<L>> {
    code: Code<L>,
    state: S,
}

impl<S, L: Literal<L>> Individual<S, L> {
    pub fn new(code: Code<L>, initial_state: S) -> Individual<S, L> {
        Individual {
            code,
            state: initial_state,
        }
    }

    pub fn get_code(&self) -> Code<L> {
        self.code.clone()
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.state
    }
}