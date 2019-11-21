//! The module with selection algorithms for type chromosomes of Vec<Float>.

use num::Float;

use crate::genetic::{Population, Selection};

/// Kill individuals if theirs gene does not lie in the specified intevals.
///
/// `G` - type of gene.
/// Returns count of the killed individuals.
pub struct CheckChromoInterval<G: Float> {
    intervals: Vec<(G, G)>,
}

impl<G: Float> CheckChromoInterval<G> {
    /// Constructor.
    ///
    /// # Parameters
    /// * `intervals` - allowed interval for every gene. Count of the genes and count of the
    /// interval must be equal.
    pub fn new(intervals: Vec<(G, G)>) -> Self {
        Self { intervals }
    }
}

impl<G: Float> Selection<Vec<G>> for CheckChromoInterval<G> {
    fn kill(&mut self, population: &mut Population<Vec<G>>) {
        for individual in population.iter_mut() {
            assert_eq!(individual.get_chromosomes().len(), self.intervals.len());

            for (chromo, interval) in individual
                .get_chromosomes()
                .iter()
                .zip(self.intervals.iter())
            {
                if !chromo.is_finite() || *chromo < interval.0 || *chromo > interval.1 {
                    individual.kill();
                    break;
                }
            }
        }
    }
}
