pub enum GeneticOperation {
    Mutation,
    Crossover,

    // TODO: ExtractFunction: a random point in the single parent's code is replaced with a new random name and the name
    // defined as the code that was at that point

    // TODO: MutationIncludingFunctions: when selecting the mutation points, all of a parent's defined_names are counted
    // as well and the mutation could occur there

    // TODO: CrossoverIncludingFunctions: when selecting the crossover points, all of each parent's defined_names are
    // counted as well and crossover could occur there
}