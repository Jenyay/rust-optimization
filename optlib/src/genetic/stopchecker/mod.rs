//! The module with structs for `StopChecker` traits.

use super::*;

/// Genetic algorithm will be stopped after specified generation (iteration).
pub struct MaxIterations {
    max_iter: usize,
}

impl MaxIterations {
    /// Constructor
    ///
    /// # Parameters
    /// * `max_iter` - how many iterations (generations) will run genetic algorithm.
    pub fn new(max_iter: usize) -> Self {
        MaxIterations { max_iter }
    }
}

impl<T> StopChecker<T> for MaxIterations {
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        population.get_iteration() >= self.max_iter
    }
}

/// Genetic algorithm will be stopped if goal function of the best individual does not change.
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
    /// * `max_iter` - how many iterations (generations) the value of goal function of the best
    /// individual may not change.
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
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        match population.get_best() {
            None => false,
            Some(individual) => {
                let best_goal = individual.get_goal();
                let delta = (best_goal - self.old_goal).abs();
                if delta > self.delta {
                    self.old_goal = best_goal;
                    self.change_iter = population.get_iteration();
                }

                (population.get_iteration() - self.change_iter) > self.max_iter
            }
        }
    }
}

/// Stop genetic algorithm if value of the goal function less of than threshold.
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
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        match population.get_best() {
            None => false,
            Some(individual) => individual.get_goal() <= self.threshold,
        }
    }
}

/// Stop genetic algorithm if ANY of stop checker returns true
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
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        for checker in &mut self.stop_checkers {
            if checker.can_stop(population) {
                return true
            }
        }

        false
    }
}

/// Stop genetic algorithm if ALL stop checkers returns true
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
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        for checker in &mut self.stop_checkers {
            if !checker.can_stop(population) {
                return false
            }
        }

        true
    }
}
