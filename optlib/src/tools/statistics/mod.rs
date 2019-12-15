//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use core::cell::RefMut;

// use num::Float;

use crate::{tools::logging::Logger, AlgorithmState, GoalValue, Solution};

/// convergence[run number][iteration]
type Convergence<T> = Vec<Vec<Option<Solution<T>>>>;

pub struct Statistics<T> {
    // index - run number
    results: Vec<Option<Solution<T>>>,

    // convergence[run number][iteration]
    convergence: Convergence<T>,
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

    pub fn get_results(&self) -> &Vec<Option<Solution<T>>> {
        &self.results
    }

    pub fn get_convergence(&self) -> &Vec<Vec<Option<Solution<T>>>> {
        &self.convergence
    }

    fn add_result(&mut self, state: &dyn AlgorithmState<T>) {
        self.results.push(state.get_best_solution().clone());
    }

    fn add_convergence(&mut self, state: &dyn AlgorithmState<T>) {
        let run_index = self.convergence.len() - 1;
        self.convergence[run_index].push(state.get_best_solution().clone());
    }
}

/// Calculate average goal function versus iteration number.
/// Average by run count.
/// Returns vector with: index - iteration, value - Option<GoalValue>.
/// Value is None if Solution is None for any running.
/// Length of result is minimal iterations count for all running.
/// # Params
/// convergence[run number][iteration]
pub fn get_average_convergence<T>(convergence: &Convergence<T>) -> Vec<Option<GoalValue>> {
    let run_count = convergence.len();
    let min_iterations = get_min_iterations(convergence);
    let mut result = Vec::with_capacity(min_iterations);

    for i in 0..min_iterations {
        let mut sum_count = 0;
        let mut sum = 0_f64;
        for run in 0..run_count {
            if let Some(solution) = &convergence[run][i] {
                sum += solution.1;
                sum_count += 1;
            }
        }

        if sum_count != 0 {
            result.push(Some(sum / (sum_count as f64)));
        } else {
            result.push(None);
        }
    }

    result
}

pub fn get_min_iterations<T>(convergence: &Convergence<T>) -> usize {
    if convergence.is_empty() {
        0
    } else {
        convergence.iter().fold(convergence[0].len(), |min_len, x| {
            if x.len() < min_len {
                x.len()
            } else {
                min_len
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_iterations_empty() {
        let convergence: Convergence<f32> = vec![];
        assert_eq!(get_min_iterations(&convergence), 0);
    }

    #[test]
    fn get_min_iterations_single_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![Some((1_f32, 0_f64))]);

        assert_eq!(get_min_iterations(&convergence), 1);
    }

    #[test]
    fn get_min_iterations_single_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
        ]);

        assert_eq!(get_min_iterations(&convergence), 3);
    }

    #[test]
    fn get_min_iterations_several_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
        ]);
        convergence.push(vec![Some((1_f32, 0_f64))]);

        assert_eq!(get_min_iterations(&convergence), 1);
    }

    #[test]
    fn get_min_iterations_several_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
        ]);
        convergence.push(vec![]);

        assert_eq!(get_min_iterations(&convergence), 0);
    }

    #[test]
    fn get_average_convergence_empty() {
        let convergence: Convergence<f32> = vec![];
        assert_eq!(get_average_convergence(&convergence), vec![]);
    }

    #[test]
    fn get_average_convergence_single_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            Some((2_f32, 20_f64)),
            Some((1_f32, 10_f64)),
        ]);

        let result = vec![
            Some(30_f64), Some(20_f64), Some(10_f64),
        ];

        assert_eq!(get_average_convergence(&convergence), result);
    }

    #[test]
    fn get_average_convergence_single_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            None,
            Some((1_f32, 10_f64)),
        ]);

        let result = vec![
            Some(30_f64), None, Some(10_f64),
        ];

        assert_eq!(get_average_convergence(&convergence), result);
    }

    #[test]
    fn get_average_convergence_several_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            Some((2_f32, 20_f64)),
            Some((1_f32, 10_f64)),
        ]);
        convergence.push(vec![
            Some((3_f32, 50_f64)),
            Some((2_f32, 40_f64)),
            Some((1_f32, 30_f64)),
        ]);

        let result = vec![
            Some(40_f64), Some(30_f64), Some(20_f64),
        ];

        assert_eq!(get_average_convergence(&convergence), result);
    }

    #[test]
    fn get_average_convergence_several_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            None,
            Some((1_f32, 10_f64)),
        ]);
        convergence.push(vec![
            Some((3_f32, 50_f64)),
            Some((2_f32, 40_f64)),
            Some((1_f32, 30_f64)),
        ]);

        let result = vec![
            Some(40_f64), Some(40_f64), Some(20_f64),
        ];

        assert_eq!(get_average_convergence(&convergence), result);
    }

    #[test]
    fn get_average_convergence_several_03() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            None,
            Some((1_f32, 10_f64)),
        ]);
        convergence.push(vec![
            Some((3_f32, 50_f64)),
            None,
            Some((1_f32, 30_f64)),
        ]);

        let result = vec![
            Some(40_f64), None, Some(20_f64),
        ];

        assert_eq!(get_average_convergence(&convergence), result);
    }
}
