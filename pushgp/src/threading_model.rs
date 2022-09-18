#[derive(Clone, Debug, PartialEq)]
pub enum ThreadingModel {
    /// Do not use multi-threading when executing the world
    None,
    
    // TODO: The threading models below require significant planning and work to safely mutate different parts of a
    // world at the same time.

    // Each Island will execute in its own thread. The parameter is the total number of islands to execute at once
    // PerIsland(usize),

    // Each Individual will execute in its own thread. The parameter is the total number of individuals to run at once.
    // PerIndividual(usize),
}
