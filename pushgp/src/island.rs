use crate::{Individual};

pub trait Island<State: std::fmt::Debug + Clone, Vm> {
    fn name() -> &'static str;
    fn pre_generation_run(&mut self);
    fn post_generation_run(&mut self);
    fn run_individual(&mut self, context: &mut Vm, individual: &mut Individual<State, Vm>);
    fn sort_individuals(&self, a: &Individual<State, Vm>, b: &Individual<State, Vm>) -> std::cmp::Ordering;
}
