use super::*;

/// MaxIteration
pub struct MaxIterations {
    max_iter: usize,
}

impl MaxIterations {
    pub fn new(max_iter: usize) -> Self {
        MaxIterations { max_iter }
    }
}

impl<T: Clone> StopChecker<T> for MaxIterations {
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        population.get_iteration() >= self.max_iter
    }
}

/// GoalNotChange
pub struct GoalNotChange {
    max_iter: usize,
    delta: f64,

    old_goal: f64,
    change_iter: usize
}

impl GoalNotChange {
    pub fn new(max_iter: usize, delta: f64) -> Self {
        GoalNotChange {
            max_iter,
            delta,
            old_goal: f64::MAX,
            change_iter: 0,
        }
    }
}

impl<T: Clone> StopChecker<T> for GoalNotChange {
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
