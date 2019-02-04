use super::super::*;

use num::Float;

pub fn kill_chromo_interval<T: Float>(
    population: &mut Population<Vec<T>>,
    minval: T,
    maxval: T,
) -> usize {
    let mut kill_count = 0;

    for individual in population.iter_mut() {
        for chromo in individual.get_chromosomes() {
            if !chromo.is_finite() || chromo < minval || chromo > maxval {
                individual.kill();
                kill_count += 1;
                break;
            }
        }
    }

    kill_count
}
