/// Defines the algorithm used when a random individual is needed from a pool of individuals that has been sorted by a
/// fitness function. The sorting algorithm defines the greatest fitness as being sorted at the end of a vector where
/// `pool.sort_by(fitness_fn)` has been called.
pub enum SelectionCurve {
    // All individuals are as likely as any other to be selected
    Fair,

    // The fitter individuals will appear much more often
    StrongPreferenceForFit,

    // The fitter individuals will appear more often
    PreferenceForFit,

    // The fitter individuals will appear a little more often
    SlightPreferenceForFit,

    // The less fit individuals will appear a little more often
    SlightPreferenceForUnfit,

    // The less fit individuals will appear more often
    PreferenceForUnfit,

    // The less fit individuals will appear much more often
    StrongPreferenceForUnfit,
}