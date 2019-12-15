pub mod logging;
pub mod stopchecker;
pub mod statistics;

use num::NumCast;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Creator to initialize vector with random values in given interval.
/// `T` - vector items type
pub struct RandomVectorCreator {
    random: ThreadRng,
}

impl RandomVectorCreator {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            random: rand::thread_rng(),
        }
    }

    pub fn create_vec<T: NumCast + PartialOrd>(&mut self, intervals: &Vec<(T, T)>) -> Vec<T> {
        for interval in intervals {
            assert!(interval.0 < interval.1);
        }

        let result = intervals
            .iter()
            .map(|(min, max)| {
                let between = Uniform::new_inclusive(min.to_f64().unwrap(), max.to_f64().unwrap());
                T::from(between.sample(&mut self.random)).unwrap()
            })
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::RandomVectorCreator;

    #[test]
    fn test_empty() {
        let intervals: Vec<(f64, f64)> = vec![];
        let mut creator = RandomVectorCreator::new();

        let result = creator.create_vec(&intervals);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_intervals() {
        let run_count = 1000;
        let intervals = vec![(0.0, 1.0), (-1.0, 1.0), (100.0, 110.0)];
        let mut creator = RandomVectorCreator::new();

        for _ in 0..run_count {
            let result: Vec<f64> = creator.create_vec(&intervals);
            assert!(result[0] >= 0.0);
            assert!(result[0] <= 1.0);

            assert!(result[1] >= -1.0);
            assert!(result[1] <= 1.0);

            assert!(result[2] >= 100.0);
            assert!(result[2] <= 110.0);
        }
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_01() {
        let intervals = vec![(1.0, 0.0)];
        let mut creator = RandomVectorCreator::new();
        creator.create_vec(&intervals);
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_02() {
        let intervals = vec![(0.0, 0.0)];
        let mut creator = RandomVectorCreator::new();
        creator.create_vec(&intervals);
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_03() {
        let intervals = vec![(0.0, 1.0), (10.0, 0.0)];
        let mut creator = RandomVectorCreator::new();
        creator.create_vec(&intervals);
    }
}
