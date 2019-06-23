//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use std::fmt::Display;
use std::time;
use std::io;

use super::super::Agent;
use super::{Logger, Population};

/// The logger prints out to stdout best chromosomes and goal function for every generation.
pub struct VerboseLogger<'a> {
    writer: &'a mut io::Write,
    precision: usize,
}

impl<'a> VerboseLogger<'a> {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(writer: &'a mut io::Write, precision: usize) -> Self {
        Self {
            writer,
            precision,
        }
    }
}

impl<'a, G: Display> Logger<Vec<G>> for VerboseLogger<'a> {
    fn next_iteration(&mut self, population: &Population<Vec<G>>) {
        if let Some(individual) = population.get_best() {
            let mut result = String::new();
            result = result + &format!("{:<8}", population.get_iteration());

            for chromo in individual.get_chromosomes() {
                result = result + &format!("  {:<20.*}", self.precision, chromo);
            }
            result = result + &format!("  {:20.*}", self.precision, individual.get_goal());

            writeln!(&mut self.writer, "{}", result).unwrap();
        }
    }
}

/// The logger print out to stdout best result and value of goal function after end of genetic algorithm running.
pub struct ResultOnlyLogger<'a> {
    writer: &'a mut io::Write,
    precision: usize,
}

impl<'a> ResultOnlyLogger<'a> {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(writer: &'a mut io::Write, precision: usize) -> Self {
        Self {
            writer,
            precision,
        }
    }
}

impl<'a, G: Display> Logger<Vec<G>> for ResultOnlyLogger<'a> {
    fn start(&mut self, _population: &Population<Vec<G>>) {}

    fn finish(&mut self, population: &Population<Vec<G>>) {
        match population.get_best() {
            None => writeln!(&mut self.writer, "Solution not found").unwrap(),
            Some(individual) => {
                let mut result = String::new();
                result = result + "Solution:\n";

                for chromo in individual.get_chromosomes() {
                    result = result + &format!("  {:.*}\n", self.precision, chromo);
                }

                result = result + "\n";
                writeln!(&mut self.writer, "{}", result).unwrap();
                writeln!(&mut self.writer, "Goal: {:.*}", self.precision, individual.get_goal()).unwrap();
            }
        }
        writeln!(&mut self.writer, "Iterations count: {}", population.get_iteration()).unwrap();
    }
}


/// The logger prints out time duration after finish of algorithm.
pub struct TimeLogger<'a> {
    writer: &'a mut io::Write,
    start_time: Option<time::Instant>,
}

impl<'a> TimeLogger<'a> {
    /// Constructor
    pub fn new(writer: &'a mut io::Write) -> Self {
        Self {
            writer,
            start_time: None,
        }
    }
}

impl<'a, G: Display> Logger<Vec<G>> for TimeLogger<'a> {
    fn resume(&mut self, _population: &Population<Vec<G>>) {
        self.start_time = Some(time::Instant::now());
    }

    fn finish(&mut self, _population: &Population<Vec<G>>) {
        let duration = self.start_time.unwrap().elapsed();
        let time_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

        writeln!(&mut self.writer, "Time elapsed: {} ms", time_ms).unwrap();
    }
}
