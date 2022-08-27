pub enum GeneticOperation {
    Mutation,
    Crossover,

    // TODO: ExtractFunction: a random point in the single parent's code is replaced with a new random name and the name
    // defined as the code that was at that point
    // BLOCKER: This feature would require inserting a NameLiteralValue into the code, which, in turn, would require
    // that the VM implements VirtualMachineMustHaveName. The only two effective ways around this are to require every
    // VM to have a name stack (which introduces other difficulties) or to wait until the `specialization` RFC becomes
    // stable (or at least close to stable) https://github.com/rust-lang/rust/issues/31844

    // TODO: MutationIncludingFunctions: when selecting the mutation points, all of a parent's defined_names are counted
    // as well and the mutation could occur there

    // TODO: CrossoverIncludingFunctions: when selecting the crossover points, all of each parent's defined_names are
    // counted as well and crossover could occur there
}