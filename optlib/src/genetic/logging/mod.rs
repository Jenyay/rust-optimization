//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use std::fmt::Display;
use std::time;

use super::super::Agent;
use super::{Logger, Population};

/// The logger prints out to stdout best chromosomes and goal function for every generation.
pub struct VerboseStdoutLogger {
    precision: usize,
}

impl VerboseStdoutLogger {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(precision: usize) -> Self {
        Self {
            precision,
        }
    }
}

impl<G: Clone + Display> Logger<Vec<G>> for VerboseStdoutLogger {
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
}

impl StdoutResultOnlyLogger {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(precision: usize) -> Self {
        Self {
            precision,
        }
    }
}

impl<G: Clone + Display> Logger<Vec<G>> for StdoutResultOnlyLogger {
    fn start(&mut self, _population: &Population<Vec<G>>) {}

    fn finish(&mut self, population: &Population<Vec<G>>) {
        match population.get_best() {
            None => println!("Solution not found"),
            Some(individual) => {
                let mut result = String::new();
                result = result + "Solution:\n";

                for chromo in individual.get_chromosomes() {
                    result = result + &format!("  {:.*}\n", self.precision, chromo);
                }

                result = result + "\n";
                println!("{}", result);
                println!("Goal: {:.*}", self.precision, individual.get_goal());
            }
        }
        println!("Iterations count: {}", population.get_iteration());
    }
}


/// The logger prints out time duration after finish of algorithm.
pub struct TimeStdoutLogger {
    start_time: Option<time::Instant>,
}

impl TimeStdoutLogger {
    /// Constructor
    pub fn new() -> Self {
        Self {
            start_time: None,
        }
    }
}

impl<G: Clone + Display> Logger<Vec<G>> for TimeStdoutLogger {
    fn resume(&mut self, _population: &Population<Vec<G>>) {
        self.start_time = Some(time::Instant::now());
    }

    fn finish(&mut self, _population: &Population<Vec<G>>) {
        let duration = self.start_time.unwrap().elapsed();
        let time_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

        println!("Time elapsed: {} ms", time_ms);
    }
}
