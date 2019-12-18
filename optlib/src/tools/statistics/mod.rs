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

/// The trait contains methods for calculate statistics for Convergance<T>
/// type Convergence<T> = Vec<Vec<Option<Solution<T>>>>;
/// convergence[run number][iteration]
pub trait StatFunctionsConvergence {
    /// Calculate average goal function versus iteration number.
    /// Average by run count.
    /// Returns vector with: index - iteration, value - Option<GoalValue>.
    /// Value is None if Solution is None for any running.
    /// Length of result is minimal iterations count for all running.
    /// # Params
    /// self[run number][iteration]
    fn get_average_convergence(&self) -> Vec<Option<GoalValue>>;
    fn get_min_iterations(&self) -> usize;
}

/// The trait contains methods for calculate statistics for Vec<Option<Solution<T>>>
/// type Solution<T> = (T, GoalValue);
pub trait StatFunctionsSolution {
    /// Calculate an average value of goal function.
    /// Returns None if `self` is empty or `self` contains `None` only.
    fn get_average_goal(&self) -> Option<GoalValue>;
}

impl<T> StatFunctionsConvergence for Convergence<T> {
    /// Calculate average goal function versus iteration number.
    /// Average by run count.
    /// Returns vector with: index - iteration, value - Option<GoalValue>.
    /// Value is None if Solution is None for any running.
    /// Length of result is minimal iterations count for all running.
    /// # Params
    /// self[run number][iteration]
    fn get_average_convergence(&self) -> Vec<Option<GoalValue>> {
        let run_count = self.len();
        let min_iterations = self.get_min_iterations();
        let mut result = Vec::with_capacity(min_iterations);

        for i in 0..min_iterations {
            let mut sum_count = 0;
            let mut sum = 0 as GoalValue;
            for run in 0..run_count {
                if let Some(solution) = &self[run][i] {
                    sum += solution.1;
                    sum_count += 1;
                }
            }

            if sum_count != 0 {
                result.push(Some(sum / (sum_count as GoalValue)));
            } else {
                result.push(None);
            }
        }

        result
    }

    fn get_min_iterations(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.iter().fold(self[0].len(), |min_len, x| {
                if x.len() < min_len {
                    x.len()
                } else {
                    min_len
                }
            })
        }
    }
}

impl<T> StatFunctionsSolution for Vec<Option<Solution<T>>> {
    /// Calculate an average value of goal function.
    /// Returns None if `self` is empty or `self` contains `None` only.
    fn get_average_goal(&self) -> Option<GoalValue> {
        let mut sum = 0 as GoalValue;
        let mut count = 0;
        for current_solution in self {
            if let Some((_, goal)) = current_solution {
                if count == 0 {
                    sum = *goal;
                    count = 1;
                } else {
                    sum += *goal;
                    count += 1;
                }
            }
        }

        if count == 0 {
            None
        } else {
            Some(sum / (count as GoalValue))
        }
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
        assert_eq!(convergence.get_min_iterations(), 0);
    }

    #[test]
    fn get_min_iterations_single_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![Some((1_f32, 0_f64))]);

        assert_eq!(convergence.get_min_iterations(), 1);
    }

    #[test]
    fn get_min_iterations_single_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
            Some((1_f32, 0_f64)),
        ]);

        assert_eq!(convergence.get_min_iterations(), 3);
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

        assert_eq!(convergence.get_min_iterations(), 1);
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

        assert_eq!(convergence.get_min_iterations(), 0);
    }

    #[test]
    fn get_average_convergence_empty() {
        let convergence: Convergence<f32> = vec![];
        assert_eq!(convergence.get_average_convergence(), vec![]);
    }

    #[test]
    fn get_average_convergence_single_01() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![
            Some((3_f32, 30_f64)),
            Some((2_f32, 20_f64)),
            Some((1_f32, 10_f64)),
        ]);

        let result = vec![Some(30_f64), Some(20_f64), Some(10_f64)];

        assert_eq!(convergence.get_average_convergence(), result);
    }

    #[test]
    fn get_average_convergence_single_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![Some((3_f32, 30_f64)), None, Some((1_f32, 10_f64))]);

        let result = vec![Some(30_f64), None, Some(10_f64)];

        assert_eq!(convergence.get_average_convergence(), result);
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

        let result = vec![Some(40_f64), Some(30_f64), Some(20_f64)];

        assert_eq!(convergence.get_average_convergence(), result);
    }

    #[test]
    fn get_average_convergence_several_02() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![Some((3_f32, 30_f64)), None, Some((1_f32, 10_f64))]);
        convergence.push(vec![
            Some((3_f32, 50_f64)),
            Some((2_f32, 40_f64)),
            Some((1_f32, 30_f64)),
        ]);

        let result = vec![Some(40_f64), Some(40_f64), Some(20_f64)];

        assert_eq!(convergence.get_average_convergence(), result);
    }

    #[test]
    fn get_average_convergence_several_03() {
        let mut convergence: Convergence<f32> = vec![];
        convergence.push(vec![Some((3_f32, 30_f64)), None, Some((1_f32, 10_f64))]);
        convergence.push(vec![Some((3_f32, 50_f64)), None, Some((1_f32, 30_f64))]);

        let result = vec![Some(40_f64), None, Some(20_f64)];

        assert_eq!(convergence.get_average_convergence(), result);
    }

    #[test]
    fn get_average_goal_empty() {
        let results: Vec<Option<Solution<f32>>> = vec![];
        assert_eq!(results.get_average_goal(), None);
    }

    #[test]
    fn get_average_goal_none_only() {
        let results: Vec<Option<Solution<f32>>> = vec![None];
        assert_eq!(results.get_average_goal(), None);
    }

    #[test]
    fn get_average_goal_single() {
        let results: Vec<Option<Solution<f32>>> = vec![Some((1.0_f32, 10.0_f64))];
        assert_eq!(results.get_average_goal(), Some(10.0_f64));
    }

    #[test]
    fn get_average_goal_several() {
        let results: Vec<Option<Solution<f32>>> =
            vec![Some((1.0_f32, 10.0_f64)), Some((2.0_f32, 30.0_f64))];
        assert_eq!(results.get_average_goal(), Some(20.0_f64));
    }
}
