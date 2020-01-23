use crate::particleswarm::PostMove;

use num::Float;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// The struct to limit the coordinates of particle.
pub struct MoveToBoundary<T> {
    intervals: Vec<(T, T)>,
}

impl<T> MoveToBoundary<T> {
    /// Constructor.
    ///
    /// # Parameters
    /// `intervals` - `intervals` - vector of tuples. Size of the vector must be equal to dimension. The first value in tuple is minimum coordinate, the second value is maximum coordinate.
    pub fn new(intervals: Vec<(T, T)>) -> Self {
        Self { intervals }
    }
}

impl<T: Float> PostMove<T> for MoveToBoundary<T> {
    fn post_move(&mut self, coordinates: &mut Vec<T>) {
        assert_eq!(coordinates.len(), self.intervals.len());

        for i in 0..coordinates.len() {
            if !coordinates[i].is_finite() {
                coordinates[i] = self.intervals[i].0;
            }

            if coordinates[i] < self.intervals[i].0 {
                coordinates[i] = self.intervals[i].0;
            }

            if coordinates[i] > self.intervals[i].1 {
                coordinates[i] = self.intervals[i].1;
            }
        }
    }
}

/// The struct to move particle to random position with given probability
pub struct RandomTeleport<T: Float + SampleUniform> {
    intervals: Vec<(T, T)>,
    probability: f32,
    random: ThreadRng,
    random_intervals: Vec<Uniform<T>>,
}

impl<T: Float + SampleUniform> RandomTeleport<T> {
    /// Constructor.
    ///
    /// # Parameters
    /// `intervals` - `intervals` - vector of tuples. Size of the vector must be equal to dimension. The first value in tuple is minimum coordinate, the second value is maximum coordinate.
    /// `probability` - probability of particle teleportation. Must be in the range [0, 1].
    pub fn new(intervals: Vec<(T, T)>, probability: f32) -> Self {
        assert!(probability >= 0_f32);
        assert!(probability <= 1_f32);
        let random_intervals = intervals
            .iter()
            .map(|(min, max)| Uniform::new_inclusive(min, max))
            .collect();
        Self {
            intervals,
            probability,
            random: rand::thread_rng(),
            random_intervals,
        }
    }
}

impl<T: Float + SampleUniform> PostMove<T> for RandomTeleport<T> {
    fn post_move(&mut self, coordinates: &mut Vec<T>) {
        assert_eq!(coordinates.len(), self.intervals.len());

        let rnd = Uniform::new(0.0_f32, 1.0_f32).sample(&mut self.random);
        let teleport = self.probability > rnd;
        if teleport {
            for i in 0..coordinates.len() {
                coordinates[i] = self.random_intervals[i].sample(&mut self.random);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::particleswarm::{postmove::MoveToBoundary, PostMove};
    use num::abs;

    #[test]
    fn test_empty() {
        let intervals: Vec<(f32, f32)> = vec![];
        let mut coordinates = vec![];

        let mut postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);
    }

    #[test]
    fn test_inside_interval() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![0.6_f32];

        let mut postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 0.6_f32) < 1e-6);
    }

    #[test]
    fn test_min() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![-1.0_f32];

        let mut postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0]) < 1e-6);
    }

    #[test]
    fn test_max() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![3.0_f32];

        let mut postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 1.0_f32) < 1e-6);
    }

    #[test]
    fn test_mix() {
        let intervals = vec![(0.0_f32, 1.0_f32), (1.0_f32, 3.0_f32), (5.0_f32, 10.0_f32)];
        let mut coordinates = vec![-3.0_f32, 4.0_f32, 6.0_f32];

        let mut postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 0.0_f32) < 1e-6);
        assert!(abs(coordinates[1] - 3.0_f32) < 1e-6);
        assert!(abs(coordinates[2] - 6.0_f32) < 1e-6);
    }
}
