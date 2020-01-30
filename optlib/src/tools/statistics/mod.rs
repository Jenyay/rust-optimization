//! The module with the loggers ready for using. The loggers implements the `Logger` trait.

use num::Float;

use crate::{tools::logging::Logger, AlgorithmState, Goal, GoalValue, Solution};

/// convergence[run number][iteration]
type Convergence<T> = Vec<Vec<Option<Solution<T>>>>;

/// The `Statistics` struct stores solutions for every algorithm running and every algorithm
/// iteration.
#[derive(Debug, Clone)]
pub struct Statistics<T> {
    /// The member stores final solution for every running. The index of vector is running number.
    results: Vec<Option<Solution<T>>>,

    /// The member stores current result for every algorithm running and every iteration. convergence[run number][iteration]
    convergence: Convergence<T>,
}

/// The `CallCountData` struct used to store call count of goal function.
#[derive(Debug, Clone)]
pub struct CallCountData(Vec<usize>);

/// The struct to calculate call count of goal function.
pub struct GoalCalcStatistics<'a, T> {
    goal: &'a mut dyn Goal<T>,
    call_count: &'a mut CallCountData,
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

/// The trait contains methods for calculate goal function statistics for Vec<Option<Solution<T>>>
/// type Solution<T> = (T, GoalValue);
pub trait StatFunctionsGoal {
    /// Calculate an average of goal function.
    /// Returns None if `self` is empty or `self` contains `None` only.
    fn get_average_goal(&self) -> Option<GoalValue>;

    /// Calculate a standard deviation of goal function.
    /// Returns None if length of `self` less 2 or `self` contains `None` only.
    fn get_standard_deviation_goal(&self) -> Option<GoalValue>;
}

/// The trait contains methods for calculate solution statistics for Vec<Option<Solution<T>>>
/// type Solution<T> = (T, GoalValue);
pub trait StatFunctionsSolution<T>: StatFunctionsGoal {
    /// Calculate an average of solutions and goal function.
    /// Returns None if `self` is empty or `self` contains `None` only.
    fn get_average(&self) -> Option<Solution<T>>;

    /// Calculate a standard deviation of solutions goal function.
    /// Returns None if length of `self` less 2 or `self` contains `None` only.
    fn get_standard_deviation(&self) -> Option<Solution<T>>;

    /// Calculate success rate between 0 and 1.
    /// Returns Some(SR) if running count > 0 and None otherwise.
    /// # Params
    /// `predicate` - function must return true for success solution and false otherwise.
    fn get_success_rate<P>(&self, predicate: P) -> Option<f64>
    where
        P: Fn(&Solution<T>) -> bool;
}

/// Create a precate for `StatFunctionsSolution<T>::get_success_rate` method.
/// The predicate compares goal function value with optimal goal.
pub fn get_predicate_success_goal<T>(
    expected_goal: GoalValue,
    delta: GoalValue,
) -> impl Fn(&Solution<T>) -> bool {
    move |(_, goal): &(T, GoalValue)| (goal - expected_goal).abs() < delta
}

/// Create a precate for `StatFunctionsSolution<T>::get_success_rate` method.
/// The predicate compares solution and valid answer.
pub fn get_predicate_success_vec_solution<T: Float>(
    expected: Vec<T>,
    delta: Vec<T>,
) -> impl Fn(&Solution<Vec<T>>) -> bool {
    assert_eq!(expected.len(), delta.len());

    move |(answer, _): &(Vec<T>, GoalValue)| {
        for (x, (e, d)) in answer.iter().zip(expected.iter().zip(delta.iter())) {
            if (*x - *e).abs() > *d {
                return false;
            }
        }
        true
    }
}

impl CallCountData {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// This method will be called before new optimization running
    pub fn next_run(&mut self) {
        self.0.push(0);
    }

    pub fn increment(&mut self) {
        if self.0.len() == 0 {
            self.0.push(0);
        }
        let index = self.0.len() - 1;

        self.0[index] += 1;
    }

    pub fn add(&mut self, n: usize) {
        if self.0.len() == 0 {
            self.0.push(0);
        }
        let index = self.0.len() - 1;

        self.0[index] += n;
    }

    /// Get call count for every running
    pub fn get_call_count(&self) -> Vec<usize> {
        self.0.clone()
    }

    /// Get average call count for all runnings
    pub fn get_average_call_count(&self) -> Option<f64> {
        let sum: usize = self.0.iter().sum();
        let count = self.0.len();
        if count == 0 {
            None
        } else {
            Some((sum as f64) / (count as f64))
        }
    }

    pub fn unite(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }
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

    pub fn unite(&mut self, mut other: Self) {
        self.results.append(&mut other.results);
        self.convergence.append(&mut other.convergence);
    }
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

impl<T> StatFunctionsGoal for Vec<Option<Solution<T>>> {
    /// Calculate an average of goal function.
    /// Returns None if `self` is empty or `self` contains `None` only.
    fn get_average_goal(&self) -> Option<GoalValue> {
        let success_solutions = self.iter().filter_map(|x| x.as_ref());
        let count = success_solutions.clone().count();
        let sum = success_solutions.fold(0 as GoalValue, |acc, (_, goal)| acc + goal);

        if count == 0 {
            None
        } else {
            Some(sum / (count as GoalValue))
        }
    }

    fn get_standard_deviation_goal(&self) -> Option<GoalValue> {
        let average = self.get_average_goal();
        if self.len() < 2 || average == None {
            return None;
        }

        let average_value = average.unwrap();

        let success_solutions = self.iter().filter_map(|x| x.as_ref());
        let count = success_solutions.clone().count();
        let sum = success_solutions.fold(0 as GoalValue, |acc, (_, goal)| {
            acc + (*goal - average_value) * (*goal - average_value)
        });

        if count < 2 {
            None
        } else {
            Some((sum / ((count - 1) as GoalValue)).sqrt())
        }
    }
}

impl<T: Float + std::fmt::Debug> StatFunctionsSolution<Vec<T>> for Vec<Option<Solution<Vec<T>>>> {
    fn get_average(&self) -> Option<Solution<Vec<T>>> {
        let goal = self.get_average_goal();
        if goal == None {
            return None;
        }

        let success_solutions = self.iter().filter_map(|x| x.as_ref());
        let count = success_solutions.clone().count();

        let mut solution: Option<Vec<T>> = None;

        for (current_solution, _) in success_solutions {
            solution = match solution {
                None => Some(current_solution.clone()),
                Some(vector) => {
                    assert_eq!(current_solution.len(), vector.len());
                    Some(vectorize(&vector, &current_solution, |x, y| *x + *y))
                }
            }
        }

        match solution {
            None => None,
            Some(vector) => {
                let result = vector
                    .iter()
                    .map(|x| *x / (T::from(count).unwrap()))
                    .collect();
                Some((result, goal.unwrap()))
            }
        }
    }

    fn get_standard_deviation(&self) -> Option<Solution<Vec<T>>> {
        if let (Some(goal_deviation), Some((solution_average, _))) =
            (self.get_standard_deviation_goal(), self.get_average())
        {
            let success_solutions = self.iter().filter_map(|x| x.as_ref());
            let count = success_solutions.clone().count();
            if count < 2 {
                return None;
            }

            let mut sum: Option<Vec<T>> = None;

            for (current_solution, _) in success_solutions {
                sum = match sum {
                    None => {
                        let diff_2 = vectorize(&current_solution, &solution_average, |x, y| {
                            (*x - *y) * (*x - *y)
                        });

                        Some(diff_2.clone())
                    }
                    Some(vector) => {
                        assert_eq!(current_solution.len(), vector.len());
                        let diff_2 = vectorize(&current_solution, &solution_average, |x, y| {
                            (*x - *y) * (*x - *y)
                        });
                        Some(vectorize(&vector, &diff_2, |x, y| *x + *y))
                    }
                };
            }

            match sum {
                None => None,
                Some(vector) => {
                    let result = vector
                        .iter()
                        .map(|x| (*x / (T::from(count - 1).unwrap())).sqrt())
                        .collect();
                    Some((result, goal_deviation))
                }
            }
        } else {
            None
        }
    }

    /// Calculate success rate between 0 and 1.
    /// Returns Some(SR) if running count > 0 and None otherwise.
    /// # Params
    /// `predicate` - function must return true for success solution and false otherwise.
    fn get_success_rate<P>(&self, predicate: P) -> Option<f64>
    where
        P: Fn(&Solution<Vec<T>>) -> bool,
    {
        let count = self.len();
        match count {
            0 => None,
            _ => {
                let success_solutions = self
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .filter(|solution| predicate(solution));
                Some(success_solutions.count() as f64 / (count as f64))
            }
        }
    }
}

fn vectorize<T>(v1: &Vec<T>, v2: &Vec<T>, func: fn(&T, &T) -> T) -> Vec<T> {
    assert_eq!(v1.len(), v2.len());
    v1.iter().zip(v2.iter()).map(|(x, y)| func(x, y)).collect()
}

pub struct StatisticsLogger<'a, T> {
    statistics: &'a mut Statistics<T>,
}

impl<'a, T> StatisticsLogger<'a, T> {
    pub fn new(statistics: &'a mut Statistics<T>) -> Self {
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

impl<'a, T> GoalCalcStatistics<'a, T> {
    pub fn new(goal: &'a mut dyn Goal<T>, call_count: &'a mut CallCountData) -> Self {
        Self { goal, call_count }
    }
}

impl<'a, T> Goal<T> for GoalCalcStatistics<'a, T> {
    fn get(&mut self, x: &T) -> GoalValue {
        self.call_count.increment();
        self.goal.get(x)
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

    #[test]
    fn get_standard_deviation_goal_empty() {
        let results: Vec<Option<Solution<f32>>> = vec![];
        assert_eq!(results.get_standard_deviation_goal(), None);
    }

    #[test]
    fn get_standard_deviation_goal_single() {
        let results: Vec<Option<Solution<f32>>> = vec![Some((1.0_f32, 10.0_f64))];
        assert_eq!(results.get_standard_deviation_goal(), None);
    }

    #[test]
    fn get_standard_deviation_goal_none_only() {
        let results: Vec<Option<Solution<f32>>> = vec![None; 10];
        assert_eq!(results.get_standard_deviation_goal(), None);
    }

    #[test]
    fn get_standard_deviation_goal_equal() {
        let results: Vec<Option<Solution<f32>>> = vec![Some((1.0_f32, 10.0_f64)); 2];
        assert!(results.get_standard_deviation_goal().unwrap().abs() < 1e-6);
    }

    #[test]
    fn get_standard_deviation_goal_several() {
        let results: Vec<Option<Solution<f32>>> = vec![
            Some((1.0_f32, 1.0_f64)),
            Some((2.0_f32, 2.0_f64)),
            Some((3.0_f32, 3.0_f64)),
        ];
        assert!((results.get_standard_deviation_goal().unwrap() - 1.0_f64).abs() < 1e-6);
    }

    #[test]
    fn get_average_vec_float_empty() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![];
        assert_eq!(results.get_average(), None);
    }

    #[test]
    fn get_average_vec_float_none_only() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![None];
        assert_eq!(results.get_average(), None);
    }

    #[test]
    fn get_average_vec_float_single() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.0_f32], 10.0_f64))];
        assert_eq!(results.get_average(), Some((vec![1.0_f32], 10.0_f64)));
    }

    #[test]
    fn get_average_vec_float_several_01() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![1.0_f32], 10.0_f64)),
            Some((vec![3.0_f32], 30.0_f64)),
        ];
        assert_eq!(results.get_average(), Some((vec![2.0_f32], 20.0_f64)));
    }

    #[test]
    fn get_average_vec_float_several_02() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            None,
            Some((vec![1.0_f32], 10.0_f64)),
            Some((vec![3.0_f32], 30.0_f64)),
        ];
        assert_eq!(results.get_average(), Some((vec![2.0_f32], 20.0_f64)));
    }

    #[test]
    fn get_average_vec_float_several_03() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![1.0_f32], 10.0_f64)),
            None,
            Some((vec![3.0_f32], 30.0_f64)),
        ];
        assert_eq!(results.get_average(), Some((vec![2.0_f32], 20.0_f64)));
    }

    #[test]
    fn get_average_vec_float_several_04() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![1.0_f32, 2.0_f32], 10.0_f64)),
            None,
            Some((vec![3.0_f32, 4.0_f32], 30.0_f64)),
        ];
        assert_eq!(
            results.get_average(),
            Some((vec![2.0_f32, 3.0_f32], 20.0_f64))
        );
    }

    #[test]
    fn get_standard_deviation_vec_float_empty() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![];
        assert_eq!(results.get_standard_deviation(), None);
    }

    #[test]
    fn get_standard_deviation_vec_float_single() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.0_f32], 10.0_f64))];
        assert_eq!(results.get_standard_deviation(), None);
    }

    #[test]
    fn get_standard_deviation_vec_float_none_only() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![None; 10];
        assert_eq!(results.get_standard_deviation(), None);
    }

    #[test]
    fn get_standard_deviation_vec_float_equal() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.0_f32], 10.0_f64)); 2];
        let deviation = results.get_standard_deviation().unwrap();
        assert!(deviation.0[0].abs() < 1e-6);
        assert!(deviation.1.abs() < 1e-6);
    }

    #[test]
    fn get_standard_deviation_goal_several_01() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![10.0_f32], 1.0_f64)),
            Some((vec![11.0_f32], 2.0_f64)),
            Some((vec![12.0_f32], 3.0_f64)),
        ];
        let deviation = results.get_standard_deviation().unwrap();

        // dbg!(deviation.clone());
        assert!((deviation.0[0] - 1.0_f32).abs() < 1e-6);
        assert!((deviation.1 - 1.0 as GoalValue).abs() < 1e-6);
    }

    #[test]
    fn get_standard_deviation_goal_several_02() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![10.0_f32, 10.0_f32], 1.0_f64)),
            Some((vec![11.0_f32, 20.0_f32], 2.0_f64)),
            Some((vec![12.0_f32, 30.0_f32], 3.0_f64)),
        ];
        let deviation = results.get_standard_deviation().unwrap();

        assert!((deviation.0[0] - 1.0_f32).abs() < 1e-6);
        assert!((deviation.0[1] - 10.0_f32).abs() < 1e-6);
        assert!((deviation.1 - 1.0 as GoalValue).abs() < 1e-6);
    }

    #[test]
    fn get_success_rate_goal_empty() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert_eq!(results.get_success_rate(&predicate), None);
    }

    #[test]
    fn get_success_rate_goal_none() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![None];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.0_f64).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_none_many() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![None; 5];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.0_f64).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_fail() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![10.0_f32], 10.0_f64))];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.0).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_01() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![10.0_f32], 1.0_f64))];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_02() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![10.0_f32], 1.00999_f64))];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_half() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![10.0_f32], 1.0_f64)),
            Some((vec![11.0_f32], 10.0_f64)),
        ];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.5).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_two_of_three_01() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![10.0_f32], 10.0_f64)),
            Some((vec![11.0_f32], 1.0_f64)),
            Some((vec![12.0_f32], 1.00001_f64)),
        ];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.66666).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_goal_full_success_two_of_three_02() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            None,
            Some((vec![11.0_f32], 1.0_f64)),
            Some((vec![12.0_f32], 1.00001_f64)),
        ];
        let predicate = get_predicate_success_goal(1.0, 0.01);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.66666).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_vec_solution_success_single() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.05_f32], 0.0_f64))];
        let expected = vec![1.0];
        let delta = vec![0.1];
        let predicate = get_predicate_success_vec_solution(expected, delta);

        assert!((results.get_success_rate(&predicate).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_vec_solution_fail_single() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.2_f32], 0.0_f64))];
        let expected = vec![1.0];
        let delta = vec![0.1];
        let predicate = get_predicate_success_vec_solution(expected, delta);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.0).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_vec_solution_half_01() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![Some((vec![1.01_f32], 0.0_f64)), None];
        let expected = vec![1.0];
        let delta = vec![0.1];
        let predicate = get_predicate_success_vec_solution(expected, delta);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.5).abs() < 1e-5);
    }

    #[test]
    fn get_success_rate_vec_solution_half_02() {
        let results: Vec<Option<Solution<Vec<f32>>>> = vec![
            Some((vec![0.01_f32, 1.01_f32], 0.0_f64)),
            Some((vec![0.01_f32, 1.2_f32], 0.0_f64)),
        ];
        let expected = vec![0.0, 1.0];
        let delta = vec![0.1, 0.1];
        let predicate = get_predicate_success_vec_solution(expected, delta);

        assert!((results.get_success_rate(&predicate).unwrap() - 0.5).abs() < 1e-5);
    }

    #[test]
    fn call_count_data_average_empty() {
        let data = CallCountData::new();
        assert_eq!(data.get_call_count(), vec![]);
        assert_eq!(data.get_average_call_count(), None);
    }

    #[test]
    fn call_count_data_average_single_zero() {
        let mut data = CallCountData::new();
        data.next_run();

        assert_eq!(data.get_call_count(), vec![0]);
        assert!((data.get_average_call_count().unwrap() - 0.0) < 1e-5);
    }

    #[test]
    fn call_count_data_average_single_one() {
        let mut data = CallCountData::new();
        data.next_run();
        data.increment();

        assert_eq!(data.get_call_count(), vec![1]);
        assert!((data.get_average_call_count().unwrap() - 1.0) < 1e-5);
    }

    #[test]
    fn call_count_data_average_single_two() {
        let mut data = CallCountData::new();
        data.next_run();
        data.add(5);

        assert_eq!(data.get_call_count(), vec![5]);
        assert!((data.get_average_call_count().unwrap() - 5.0) < 1e-5);
    }

    #[test]
    fn call_count_data_average_several() {
        let mut data = CallCountData::new();
        data.next_run();
        data.add(5);
        data.next_run();
        data.add(7);

        assert_eq!(data.get_call_count(), vec![5, 7]);
        assert!((data.get_average_call_count().unwrap() - 6.0) < 1e-5);
    }

    #[test]
    fn statistics_convergence_unite_empty() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let stat_2: Statistics<f32> = Statistics::new();

        let valid_results: Vec<Option<Solution<f32>>> = vec![];
        let valid_convergence: Convergence<f32> = vec![];

        stat_1.unite(stat_2);
        assert_eq!(stat_1.results, valid_results);
        assert_eq!(stat_1.convergence, valid_convergence);
    }

    #[test]
    fn statistics_unite_results_01() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let stat_2: Statistics<f32> = Statistics::new();

        let valid_results: Vec<Option<Solution<f32>>> = vec![Some((1.0_f32, 0.0))];
        stat_1.results.push(Some((1.0_f32, 0.0)));

        stat_1.unite(stat_2);
        assert_eq!(stat_1.results, valid_results);
    }

    #[test]
    fn statistics_unite_results_02() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let mut stat_2: Statistics<f32> = Statistics::new();

        let valid_results: Vec<Option<Solution<f32>>> = vec![Some((1.0_f32, 0.0))];
        stat_2.results.push(Some((1.0_f32, 0.0)));

        stat_1.unite(stat_2);
        assert_eq!(stat_1.results, valid_results);
    }

    #[test]
    fn statistics_unite_results_03() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let mut stat_2: Statistics<f32> = Statistics::new();

        let valid_results: Vec<Option<Solution<f32>>> =
            vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))];

        stat_1.results.push(Some((1.0_f32, 0.0)));
        stat_2.results.push(Some((2.0_f32, 1.0)));

        stat_1.unite(stat_2);
        assert_eq!(stat_1.results, valid_results);
    }

    #[test]
    fn convergence_unite_01() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let stat_2: Statistics<f32> = Statistics::new();

        let valid_convergence: Convergence<f32> =
            vec![vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))]];

        stat_1
            .convergence
            .push(vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))]);

        stat_1.unite(stat_2);
        assert_eq!(stat_1.convergence, valid_convergence);
    }

    #[test]
    fn convergence_unite_02() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let mut stat_2: Statistics<f32> = Statistics::new();

        let valid_convergence: Convergence<f32> =
            vec![vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))]];

        stat_2
            .convergence
            .push(vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))]);

        stat_1.unite(stat_2);
        assert_eq!(stat_1.convergence, valid_convergence);
    }

    #[test]
    fn convergence_unite_04() {
        let mut stat_1: Statistics<f32> = Statistics::new();
        let mut stat_2: Statistics<f32> = Statistics::new();

        let valid_convergence: Convergence<f32> = vec![
            vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))],
            vec![Some((10.0_f32, 10.0)), Some((20.0_f32, 2.0))],
        ];

        stat_1
            .convergence
            .push(vec![Some((1.0_f32, 0.0)), Some((2.0_f32, 1.0))]);

        stat_2
            .convergence
            .push(vec![Some((10.0_f32, 10.0)), Some((20.0_f32, 2.0))]);

        stat_1.unite(stat_2);
        assert_eq!(stat_1.convergence, valid_convergence);
    }

    #[test]
    fn call_count_data_unite_empty() {
        let mut call_count_1 = CallCountData::new();
        let call_count_2 = CallCountData::new();

        let valid_call_count_stat: Vec<usize> = vec![];

        call_count_1.unite(call_count_2);

        assert_eq!(call_count_1.0, valid_call_count_stat);
    }

    #[test]
    fn call_count_data_unite_01() {
        let mut call_count_1 = CallCountData::new();
        let call_count_2 = CallCountData::new();

        let valid_call_count_stat: Vec<usize> = vec![100];

        call_count_1.0.push(100);
        call_count_1.unite(call_count_2);

        assert_eq!(call_count_1.0, valid_call_count_stat);
    }

    #[test]
    fn call_count_data_unite_02() {
        let mut call_count_1 = CallCountData::new();
        let mut call_count_2 = CallCountData::new();

        let valid_call_count_stat: Vec<usize> = vec![100];

        call_count_2.0.push(100);
        call_count_1.unite(call_count_2);

        assert_eq!(call_count_1.0, valid_call_count_stat);
    }

    #[test]
    fn call_count_data_unite_03() {
        let mut call_count_1 = CallCountData::new();
        let mut call_count_2 = CallCountData::new();

        let valid_call_count_stat: Vec<usize> = vec![100, 200];

        call_count_1.0.push(100);
        call_count_2.0.push(200);

        call_count_1.unite(call_count_2);

        assert_eq!(call_count_1.0, valid_call_count_stat);
    }
}
