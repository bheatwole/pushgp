use crate::{Code, Name, RunResult};
use fnv::FnvHashMap;

pub struct Individual<R: RunResult> {
    code: Code,
    defined_names: FnvHashMap<Name, Code>,
    run_result: Option<R>,
}

impl<R: RunResult> Individual<R> {
    pub fn new(code: Code, defined_names: FnvHashMap<Name, Code>, initial_run_result: Option<R>) -> Individual<R> {
        Individual { code, defined_names, run_result: initial_run_result }
    }

    /// Borrows the Individual's code
    pub fn get_code(&self) -> &Code {
        &self.code
    }

    /// Sets the Individual's code to a new value
    pub fn set_code(&mut self, code: Code) {
        self.code = code
    }

    /// Borrows the HashMap of names that are defined for this Individual
    pub fn get_defined_names(&self) -> &FnvHashMap<Name, Code> {
        &self.defined_names
    }

    /// Mutably borrows the HashMap of names that are defined for this Individual, allowing for changes
    pub fn get_defined_names_mut(&mut self) -> &mut FnvHashMap<Name, Code> {
        &mut self.defined_names
    }

    /// Replaces the defined names for this Individual with a specific list
    pub fn set_defined_names(&mut self, defined_names: FnvHashMap<Name, Code>) {
        self.defined_names = defined_names;
    }

    /// Examines `names_to_set` and if they are defined in `search_map`, sets the same name in this Individual with a
    /// clone of the code from `search_map`.
    pub fn set_specific_defined_names(&mut self, names_to_set: &[Name], search_map: &FnvHashMap<Name, Code>) {
        for name in names_to_set {
            if let Some(code) = search_map.get(name) {
                self.defined_names.insert(name.clone(), code.clone());
            }
        }
    }

    /// Borrows the current RunResult for the Individual
    pub fn get_run_result(&self) -> Option<&R> {
        self.run_result.as_ref()
    }

    /// Mutably borrows the current RunResult for the Individual, allowing for changes to results
    pub fn get_run_result_mut(&mut self) -> Option<&mut R> {
        self.run_result.as_mut()
    }

    /// Replaces the RunResult for this Individual
    pub fn set_run_result(&mut self, run_result: Option<R>) {
        self.run_result = run_result;
    }
}

impl<R: RunResult> Clone for Individual<R> {
    fn clone(&self) -> Self {
        Self { code: self.code.clone(), defined_names: self.defined_names.clone(), run_result: self.run_result.clone() }
    }
}
