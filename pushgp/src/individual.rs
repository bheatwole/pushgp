use crate::Code;
use fnv::FnvHashMap;

pub struct Individual<S, Vm> {
    code: Code<Vm>,
    defined_names: FnvHashMap<String, Code<Vm>>,
    state: S,
}

impl<S, Vm: 'static> Individual<S, Vm> {
    pub fn new(code: Code<Vm>, defined_names: FnvHashMap<String, Code<Vm>>, initial_state: S) -> Individual<S, Vm> {
        Individual { code, defined_names, state: initial_state }
    }

    pub fn get_code(&self) -> Code<Vm> {
        self.code.clone()
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.state
    }
}
