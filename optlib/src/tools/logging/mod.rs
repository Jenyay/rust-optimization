//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use std::fmt::Display;
use std::io;
use std::time;

use crate::AlgorithmState;

/// The logging trait for algorithm with the agents.
///
/// `T` - type of a point in the search space for goal function.
pub trait Logger<T> {
    /// Will be called after algorithm initializing.
    fn start(&mut self, _state: &dyn AlgorithmState<T>) {}

    /// Will be called before run algorithm (possibly after result algorithm after pause).
    fn resume(&mut self, _state: &dyn AlgorithmState<T>) {}

    /// Will be called in the end of iteration.
    fn next_iteration(&mut self, _state: &dyn AlgorithmState<T>) {}

    /// Will be called when algorithm will be stopped.
    fn finish(&mut self, _state: &dyn AlgorithmState<T>) {}

    // fn as_any(&self) -> &dyn Any;
}

/// The logger prints out current solution and goal function for every iteration.
pub struct VerboseLogger<'a> {
    writer: &'a mut dyn io::Write,
    precision: usize,
}

impl<'a> VerboseLogger<'a> {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(writer: &'a mut dyn io::Write, precision: usize) -> Self {
        Self { writer, precision }
    }
}

impl<'a, T: Display> Logger<Vec<T>> for VerboseLogger<'a> {
    fn next_iteration(&mut self, state: &dyn AlgorithmState<Vec<T>>) {
        if let Some((solution, goal)) = state.get_best_solution() {
            let mut result = String::new();
            result = result + &format!("{:<8}", state.get_iteration());

            for x in solution {
                result = result + &format!("  {:<20.*}", self.precision, x);
            }
            result = result + &format!("  {:20.*}", self.precision, goal);

            writeln!(&mut self.writer, "{}", result).unwrap();
        }
    }
}

/// The logger print out to stdout best result and value of goal function after end of genetic algorithm running.
pub struct ResultOnlyLogger<'a> {
    writer: &'a mut dyn io::Write,
    precision: usize,
}

impl<'a> ResultOnlyLogger<'a> {
    /// Constructor
    ///
    /// # Parameters
    /// * `precision` - count of the digits after comma for float numbers.
    pub fn new(writer: &'a mut dyn io::Write, precision: usize) -> Self {
        Self { writer, precision }
    }
}

impl<'a, T: Display> Logger<Vec<T>> for ResultOnlyLogger<'a> {
    fn start(&mut self, _state: &dyn AlgorithmState<Vec<T>>) {}

    fn finish(&mut self, state: &dyn AlgorithmState<Vec<T>>) {
        match state.get_best_solution() {
            None => writeln!(&mut self.writer, "Solution not found").unwrap(),
            Some((solution, goal)) => {
                let mut result = String::new();
                result = result + "Solution:\n";

                for x in solution {
                    result = result + &format!("  {:.*}\n", self.precision, x);
                }

                result = result + "\n";
                writeln!(&mut self.writer, "{}", result).unwrap();
                writeln!(&mut self.writer, "Goal: {:.*}", self.precision, goal).unwrap();
            }
        }
        writeln!(
            &mut self.writer,
            "Iterations count: {}",
            state.get_iteration()
        )
        .unwrap();
    }
}

/// The logger prints out time duration after finish of algorithm.
pub struct TimeLogger<'a> {
    writer: &'a mut dyn io::Write,
    start_time: Option<time::Instant>,
}

impl<'a> TimeLogger<'a> {
    /// Constructor
    pub fn new(writer: &'a mut dyn io::Write) -> Self {
        Self {
            writer,
            start_time: None,
        }
    }
}

impl<'a, T: Display> Logger<Vec<T>> for TimeLogger<'a> {
    fn resume(&mut self, _state: &dyn AlgorithmState<Vec<T>>) {
        self.start_time = Some(time::Instant::now());
    }

    fn finish(&mut self, _state: &dyn AlgorithmState<Vec<T>>) {
        let duration = self.start_time.unwrap().elapsed();
        let time_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

        writeln!(&mut self.writer, "Time elapsed: {} ms", time_ms).unwrap();
    }
}
