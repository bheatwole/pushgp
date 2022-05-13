use crate::{Individual, LiteralEnum};

pub trait Island<C, S, L: LiteralEnum<L>> {
    fn name() -> &'static str;
    fn pre_generation_run(&mut self);
    fn post_generation_run(&mut self);
    fn run_individual(&mut self, context: &C, individual: &mut Individual<S, L>);
    fn sort_individuals(&self, a: &Individual<S, L>, b: &Individual<S, L>) -> std::cmp::Ordering;
}