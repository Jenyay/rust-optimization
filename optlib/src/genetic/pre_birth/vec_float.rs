//! The module with PreBirth trait implementations for chromosomes of Vec<T> type.

use num::Float;

use crate::genetic::{Population, PreBirth};

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

impl<G: Float> PreBirth<Vec<G>> for CheckChromoInterval<G> {
    fn pre_birth(&mut self, _population: &Population<Vec<G>>, new_chromosomes: &mut Vec<Vec<G>>) {
        new_chromosomes.retain(|chromosomes| self.check_chromo(chromosomes));
    }
}

impl<G: Float> CheckChromoInterval<G> {
    fn check_chromo(&mut self, chromosomes: &Vec<G>) -> bool {
        assert_eq!(chromosomes.len(), self.intervals.len());

        for (chromo, interval) in chromosomes.iter().zip(self.intervals.iter()) {
            if !chromo.is_finite() || *chromo < interval.0 || *chromo > interval.1 {
                return false;
            }
        }

        true
    }
}
