use crate::particleswarm::PostMove;

use num::Float;

pub struct NonePostMove {}

impl NonePostMove {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> PostMove<T> for NonePostMove {
    fn post_move(&self, _coordinates: &mut Vec<T>) {}
}

pub struct MoveToBoundary<T> {
    intervals: Vec<(T, T)>,
}

impl<T> MoveToBoundary<T> {
    pub fn new(intervals: Vec<(T, T)>) -> Self {
        Self { intervals }
    }
}

impl<T: Float> PostMove<T> for MoveToBoundary<T> {
    fn post_move(&self, coordinates: &mut Vec<T>) {
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

#[cfg(test)]
mod tests {
    use crate::particleswarm::{postmove::MoveToBoundary, PostMove};
    use num::abs;

    #[test]
    fn test_empty() {
        let intervals: Vec<(f32, f32)> = vec![];
        let mut coordinates = vec![];

        let postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);
    }

    #[test]
    fn test_inside_interval() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![0.6_f32];

        let postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 0.6_f32) < 1e-6);
    }

    #[test]
    fn test_min() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![-1.0_f32];

        let postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0]) < 1e-6);
    }

    #[test]
    fn test_max() {
        let intervals = vec![(0.0_f32, 1.0_f32)];
        let mut coordinates = vec![3.0_f32];

        let postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 1.0_f32) < 1e-6);
    }

    #[test]
    fn test_mix() {
        let intervals = vec![(0.0_f32, 1.0_f32), (1.0_f32, 3.0_f32), (5.0_f32, 10.0_f32)];
        let mut coordinates = vec![-3.0_f32, 4.0_f32, 6.0_f32];

        let postmove = MoveToBoundary::new(intervals);
        postmove.post_move(&mut coordinates);

        assert!(abs(coordinates[0] - 0.0_f32) < 1e-6);
        assert!(abs(coordinates[1] - 3.0_f32) < 1e-6);
        assert!(abs(coordinates[2] - 6.0_f32) < 1e-6);
    }
}
