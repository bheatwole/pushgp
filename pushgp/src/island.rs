use crate::{Individual, VirtualMachine};

pub trait Island<RunResult: std::fmt::Debug + Clone, Vm: VirtualMachine> {
    fn pre_generation_run(&mut self);
    fn post_generation_run(&mut self);
    fn run_individual(&mut self, context: &mut Vm, individual: &mut Individual<RunResult, Vm>);
    fn sort_individuals(&self, a: &Individual<RunResult, Vm>, b: &Individual<RunResult, Vm>) -> std::cmp::Ordering;
}
