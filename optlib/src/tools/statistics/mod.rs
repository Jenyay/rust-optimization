//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use core::cell::RefMut;

use crate::{tools::logging::Logger, AlgorithmState, Solution};

pub struct Statistics<T> {
    // index - run number
    results: Vec<Option<Solution<T>>>,

    // convergence[run number][iteration]
    convergence: Vec<Vec<Option<Solution<T>>>>,
}

impl<T: Clone> Statistics<T> {
    pub fn new() -> Self {
        Self {
            results: vec![],
            convergence: vec![],
        }
    }

    pub fn get_run_count(&self) -> usize {
        self.results.len()
    }

    pub fn get_results(&self) -> Vec<Option<Solution<T>>> {
        self.results.clone()
    }

    pub fn get_convergence(&self) -> Vec<Vec<Option<Solution<T>>>> {
        self.convergence.clone()
    }

    fn add_result(&mut self, state: &dyn AlgorithmState<T>) {
        self.results.push(state.get_best_solution().clone());
    }

    fn add_convergence(&mut self, state: &dyn AlgorithmState<T>) {
        let run_index = self.convergence.len() - 1;
        self.convergence[run_index].push(state.get_best_solution().clone());
    }
}

pub struct StatisticsLogger<'a, T> {
    statistics: RefMut<'a, Statistics<T>>,
}

impl<'a, T> StatisticsLogger<'a, T> {
    pub fn new(statistics: RefMut<'a, Statistics<T>>) -> Self {
        Self { statistics }
    }
}

impl<'a, T: Clone> Logger<T> for StatisticsLogger<'a, T> {
    /// Will be called after algorithm initializing.
    fn start(&mut self, _state: &dyn AlgorithmState<T>) {
        self.statistics.convergence.push(vec![]);
    }

    /// Will be called before run algorithm (possibly after result algorithm after pause).
    fn resume(&mut self, _state: &dyn AlgorithmState<T>) {}

    /// Will be called in the end of iteration.
    fn next_iteration(&mut self, state: &dyn AlgorithmState<T>) {
        self.statistics.add_convergence(state);
    }

    /// Will be called when algorithm will be stopped.
    fn finish(&mut self, state: &dyn AlgorithmState<T>) {
        self.statistics.add_result(state);
    }
}
