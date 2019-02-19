//! The module with ready for using loggers. The loggers implements the `Logger` trait.

use std::fmt::Display;
use std::time;

use super::super::Agent;
use super::{Logger, Population};

/// The logger prints out to stdout best chromosomes and goal function for every generation.
pub struct VerboseStdoutLogger {
    precision: usize,
    start_time: Option<time::Instant>,
}

impl VerboseStdoutLogger {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - number of digit after comma for float numbers.
    pub fn new(precision: usize) -> Self {
        Self {
            precision,
            start_time: None,
        }
    }
}

impl<G: Clone + Display> Logger<Vec<G>> for VerboseStdoutLogger {
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

/// The logger print out to stdout best result and value of goal function after end of genetic algorithm running.
pub struct StdoutResultOnlyLogger {
    precision: usize,
    start_time: Option<time::Instant>,
}

impl StdoutResultOnlyLogger {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - number of digit after comma for float numbers.
    pub fn new(precision: usize) -> Self {
        Self {
            precision,
            start_time: None,
        }
    }
}

impl<G: Clone + Display> Logger<Vec<G>> for StdoutResultOnlyLogger {
    fn start(&mut self, _population: &Population<Vec<G>>) {}

    fn resume(&mut self, _population: &Population<Vec<G>>) {
        self.start_time = Some(time::Instant::now());
    }

    fn finish(&mut self, population: &Population<Vec<G>>) {
        let duration = self.start_time.unwrap().elapsed();
        let time_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

        match population.get_best() {
            None => println!("Solution not found"),
            Some(individual) => {
                let mut result = String::new();
                result = result + "Solution:\n    ";

                for chromo in individual.get_chromosomes() {
                    result = result + &format!("  {:<20.*}", self.precision, chromo);
                }

                result = result + "\n";
                println!("{}", result);
                println!("Goal: {:.*}", self.precision, individual.get_goal());
            }
        }
        println!("Iterations count: {}", population.get_iteration());
        println!("Time elapsed: {} ms", time_ms);
    }

    fn next_iteration(&mut self, _population: &Population<Vec<G>>) {}
}
