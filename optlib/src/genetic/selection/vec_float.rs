//! The module with selection algorithms for type chromosomes of Vec<Float>.

use super::super::*;

use num::Float;

/// Kill individuals if theirs gene does not lie in the specified inteval.
///
/// `G` - type of gene.
/// Returns count of the killed individuals.
pub struct CheckChromoInterval<G: Float> {
    interval: (G, G),
}

impl<G: Float> CheckChromoInterval<G> {
    pub fn new(interval: (G, G)) -> Self {
        Self { interval }
    }
}

impl<G: Float> Selection<Vec<G>> for CheckChromoInterval<G> {
    fn kill(&mut self, population: &mut Population<Vec<G>>) {
        for individual in population.iter_mut() {
            for chromo in individual.get_chromosomes() {
                if !chromo.is_finite() || *chromo < self.interval.0 || *chromo > self.interval.1 {
                    individual.kill();
                    break;
                }
            }
        }
    }
}
