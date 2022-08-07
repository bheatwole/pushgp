use crate::{Code, VirtualMachine, Name};
use fnv::FnvHashMap;

#[derive(Clone)]
pub struct Individual<RunResult, Vm: VirtualMachine> {
    code: Code<Vm>,
    defined_names: FnvHashMap<Name, Code<Vm>>,
    run_result: Option<RunResult>,
}

impl<RunResult, Vm: VirtualMachine> Individual<RunResult, Vm> {
    pub fn new(
        code: Code<Vm>,
        defined_names: FnvHashMap<Name, Code<Vm>>,
        initial_run_result: Option<RunResult>,
    ) -> Individual<RunResult, Vm> {
        Individual { code, defined_names, run_result: initial_run_result }
    }

    pub fn get_code(&self) -> &Code<Vm> {
        &self.code
    }

    pub fn set_code(&mut self, code: Code<Vm>) {
        self.code = code
    }

    pub fn get_defined_names(&self) -> &FnvHashMap<Name, Code<Vm>> {
        &self.defined_names
    }

    pub fn get_defined_names_mut(&mut self) -> &mut FnvHashMap<Name, Code<Vm>> {
        &mut self.defined_names
    }

    pub fn set_defined_names(&mut self, defined_names: FnvHashMap<Name, Code<Vm>>) {
        self.defined_names = defined_names;
    }

    pub fn get_run_result(&self) -> &Option<RunResult> {
        &self.run_result
    }

    pub fn get_run_result_mut(&mut self) -> &mut Option<RunResult> {
        &mut self.run_result
    }

    pub fn set_run_result(&mut self, run_result: Option<RunResult>) {
        self.run_result = run_result;
    }
}
