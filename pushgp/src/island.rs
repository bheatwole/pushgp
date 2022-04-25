use crate::Individual;

pub trait Island<C, S> {
    fn name() -> &'static str;
    fn pre_generation_run(&mut self);
    fn post_generation_run(&mut self);
    fn run_individual(&mut self, context: &C, individual: &mut Individual<S>);
    fn sort_individuals(&self, a: &Individual<S>, b: &Individual<S>) -> std::cmp::Ordering;
}