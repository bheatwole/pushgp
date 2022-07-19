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

impl SelectionCurve {
    /// Randomly selects a value in the range [0 .. number_of_individuals] according to the SelectionCurve properties
    pub fn pick_one_index<R: rand::Rng>(&self, rng: &mut R, number_of_individuals: usize) -> usize {
        // Pick a value in the range of (0.0 .. 1.0] (includes zero, but not one). This behavior is part of the
        // guarantee of the rand::distributions::Standard spec
        let pick: f64 = rng.gen();

        // Multiply the pick by the number of individuals and turn it into an integer
        (pick * number_of_individuals as f64).floor() as usize
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::*;

    
    #[test]
    fn fair_selection_curve() { 
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1234);
        let mut buckets = vec![0usize; 100];

        // Pick from 0 to 100, 100_000 times
        for _ in 0..100_000 {
            let pick = SelectionCurve::Fair.pick_one_index(&mut rng, 100);
            buckets[pick] += 1;
        }

        // Each bucket should have at least 900 and no more than 1100
        for (i, &bucket) in buckets.iter().enumerate() {
            assert!(bucket >= 900 && bucket <= 1100, "bucket[{}] had {}", i, bucket);
        }
    }
}