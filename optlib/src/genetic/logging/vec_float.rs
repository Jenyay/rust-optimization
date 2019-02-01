use std::fmt::Display;

use super::super::*;

use num::Float;

pub struct StdoutLogger {
    precision: usize,
}

impl StdoutLogger {
    pub fn new(precision: usize) -> Self {
        Self { precision }
    }
}

impl<G: Float + Display> Logger<Vec<G>> for StdoutLogger {
    fn start(&mut self, _population: &Population<Vec<G>>) {}
    fn resume(&mut self, _population: &Population<Vec<G>>) {}
    fn finish(&mut self, _population: &Population<Vec<G>>) {}

    fn next_iteration(&mut self, population: &Population<Vec<G>>) {
        if let Some(individual) = population.get_best() {
            let mut result = String::new();
            result = result + &format!("{:<8}", population.get_iteration());

            for chromo in individual.get_chromosomes() {
                result = result + &format!("  {:<20.*}", self.precision, chromo);
            }
            result = result + &format!("  {:20.*}", self.precision, individual.get_goal());

            println!("{}", result);
        }
    }
}
