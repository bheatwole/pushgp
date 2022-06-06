use crate::{Context, Individual};

pub trait Island<State: std::fmt::Debug + Clone> {
    fn name() -> &'static str;
    fn pre_generation_run(&mut self);
    fn post_generation_run(&mut self);
    fn run_individual(&mut self, context: &mut Context<State>, individual: &mut Individual<State>);
    fn sort_individuals(&self, a: &Individual<State>, b: &Individual<State>) -> std::cmp::Ordering;
}
