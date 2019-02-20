//! The module with selection algorithms for type chromosomes of Vec<Float>.

use super::super::*;

use num::Float;

/// Kill individuals if theirs gene does not lie in the specified inteval.
///
/// `G` - type of gene.
/// Returns count of the killed individuals.
pub fn kill_chromo_interval<G: Float>(
    population: &mut Population<Vec<G>>,
    minval: G,
    maxval: G,
) -> usize {
    let mut kill_count = 0;

    for individual in population.iter_mut() {
        for chromo in individual.get_chromosomes() {
            if !chromo.is_finite() || *chromo < minval || *chromo > maxval {
                individual.kill();
                kill_count += 1;
                break;
            }
        }
    }

    kill_count
}
