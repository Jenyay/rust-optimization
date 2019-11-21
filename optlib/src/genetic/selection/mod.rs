//! The module with selection algorithms.
//!
//! The algoritms must kill individuals which does not go to
//! next generation. The algorithm must call `kill()` method for such individuals.

pub mod vec_float;

use crate::genetic::{Population, Selection};

/// Kill individuals if value of theirs fitness (goal function) is NaN.
/// Returns count of killed individuals.
pub struct KillFitnessNaN;

impl<T: Clone> Selection<T> for KillFitnessNaN {
    fn kill(&mut self, population: &mut Population<T>) {
        for individual in population.iter_mut() {
            if !individual.get_fitness().is_finite() {
                individual.kill();
            }
        }
    }
}

impl KillFitnessNaN {
    /// Constructor.
    pub fn new() -> Self {
        Self
    }
}

pub struct LimitPopulation {
    max_count: usize,
}

impl LimitPopulation {
    /// Constructor.
    pub fn new(max_count: usize) -> Self {
        Self { max_count }
    }

    pub fn set_limit(&mut self, max_count: usize) {
        self.max_count = max_count;
    }
}

impl<T: Clone> Selection<T> for LimitPopulation {
    fn kill(&mut self, population: &mut Population<T>) {
        let alive_count = population.len_alive();
        if alive_count > self.max_count {
            kill_worst(population, alive_count - self.max_count);
        }
    }
}

/// Function to kill worst individuals in population.
/// `count` - how many individuals must be killed.
pub fn kill_worst<T: Clone>(population: &mut Population<T>, count: usize) {
    // List of indexes of individuals in population to be kill
    let mut kill_list: Vec<usize> = Vec::with_capacity(count);
    kill_list.push(0);

    // Index of the items in kill_list with best fitness
    let mut best_index = 0;
    let mut best_fitness = population[kill_list[best_index]].get_fitness();

    for n in 1..population.len() {
        if !population[n].is_alive() {
            continue;
        }

        if kill_list.len() < count {
            kill_list.push(n);
            if population[n].get_fitness() < best_fitness {
                best_index = kill_list.len() - 1;
            }
        } else {
            if population[n].get_fitness() > best_fitness {
                kill_list[best_index] = n;

                // Find new best item
                best_index = 0;
                best_fitness = population[kill_list[best_index]].get_fitness();
                for m in 1..kill_list.len() {
                    if population[kill_list[m]].get_fitness() < best_fitness {
                        best_index = m;
                        best_fitness = population[kill_list[best_index]].get_fitness();
                    }
                }
            }
        }
    }

    for n in kill_list {
        population[n].kill();
    }
}
