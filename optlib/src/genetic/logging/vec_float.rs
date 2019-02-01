use std::fmt::Display;
use std::time;

use super::super::*;

use num::Float;

pub struct StdoutLogger {
    precision: usize,
    start_time: Option<time::Instant>,
}

impl StdoutLogger {
    pub fn new(precision: usize) -> Self {
        Self {
            precision,
            start_time: None,
        }
    }
}

impl<G: Float + Display> Logger<Vec<G>> for StdoutLogger {
    fn start(&mut self, _population: &Population<Vec<G>>) {}

    fn resume(&mut self, _population: &Population<Vec<G>>) {
        self.start_time = Some(time::Instant::now());
    }

    fn finish(&mut self, _population: &Population<Vec<G>>) {
        let duration = self.start_time.unwrap().elapsed();
        let time_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

        println!("Time elapsed: {} ms", time_ms);
    }

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
