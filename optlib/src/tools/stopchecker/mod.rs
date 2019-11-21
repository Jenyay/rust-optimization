use std::f64;

use super::super::AlgorithmState;

/// The trait with break criterion optimization algorithm.
///
/// `T` - type of a point in the search space for goal function.
pub trait StopChecker<T> {
    /// The method must return true if algorithm must be stopped.
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool;
}

/// Stop the algorithm if ANY of stop checker returns true
pub struct CompositeAny<T> {
    stop_checkers: Vec<Box<dyn StopChecker<T>>>,
}

impl<T> CompositeAny<T> {
    /// Constructor
    pub fn new(stop_checkers: Vec<Box<dyn StopChecker<T>>>) -> Self {
        assert!(stop_checkers.len() != 0);
        Self { stop_checkers }
    }
}

impl<T> StopChecker<T> for CompositeAny<T> {
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool {
        for checker in &mut self.stop_checkers {
            if checker.can_stop(state) {
                return true;
            }
        }

        false
    }
}

/// Stop the algorithm if ALL stop checkers returns true
pub struct CompositeAll<T> {
    stop_checkers: Vec<Box<dyn StopChecker<T>>>,
}

impl<T> CompositeAll<T> {
    /// Constructor
    pub fn new(stop_checkers: Vec<Box<dyn StopChecker<T>>>) -> Self {
        assert!(stop_checkers.len() != 0);
        Self { stop_checkers }
    }
}

impl<T> StopChecker<T> for CompositeAll<T> {
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool {
        for checker in &mut self.stop_checkers {
            if !checker.can_stop(state) {
                return false;
            }
        }

        true
    }
}

/// The algorithm will be stopped after specified iteration.
pub struct MaxIterations {
    max_iter: usize,
}

impl MaxIterations {
    /// Constructor
    ///
    /// # Parameters
    /// * `max_iter` - how many iterations will run the algorithm.
    pub fn new(max_iter: usize) -> Self {
        MaxIterations { max_iter }
    }
}

impl<T> StopChecker<T> for MaxIterations {
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool {
        state.get_iteration() >= self.max_iter
    }
}

/// The algorithm will be stopped if the best goal function does not change.
pub struct GoalNotChange {
    max_iter: usize,
    delta: f64,

    old_goal: f64,
    change_iter: usize,
}

impl GoalNotChange {
    /// Constructor.
    ///
    /// # Parameters
    /// * `max_iter` - how many iterations the value of goal function of the best
    /// solution may not change.
    /// * `delta` - small value. The change of goal function is not considered if the change less
    /// of `delta`.
    pub fn new(max_iter: usize, delta: f64) -> Self {
        GoalNotChange {
            max_iter,
            delta,
            old_goal: f64::MAX,
            change_iter: 0,
        }
    }
}

impl<T> StopChecker<T> for GoalNotChange {
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool {
        match state.get_best_solution() {
            None => false,
            Some((_, best_goal)) => {
                let delta = (best_goal - self.old_goal).abs();
                if delta > self.delta {
                    self.old_goal = best_goal;
                    self.change_iter = state.get_iteration();
                }

                (state.get_iteration() - self.change_iter) > self.max_iter
            }
        }
    }
}

/// Stop the algorithm if value of the goal function less of than threshold.
pub struct Threshold {
    threshold: f64,
}

impl Threshold {
    /// Constructor.
    ///
    /// # Parameters
    /// * `threshold` - min value of the goal function
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

impl<T> StopChecker<T> for Threshold {
    fn can_stop(&mut self, state: &dyn AlgorithmState<T>) -> bool {
        match state.get_best_solution() {
            None => false,
            Some((_, goal)) => goal <= self.threshold,
        }
    }
}
