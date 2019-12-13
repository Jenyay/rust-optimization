use num::Float;

use crate::particleswarm::PostSpeedCalc;

/// The trait to restrict value for every dimension of speed
pub struct MaxSpeedDimensions<T> {
    max_speed: Vec<T>,
}

impl<T> MaxSpeedDimensions<T> {
    pub fn new(max_speed: Vec<T>) -> Self {
        Self { max_speed }
    }
}

impl<T: Float> PostSpeedCalc<T> for MaxSpeedDimensions<T> {
    fn correct_speed(&mut self, speed: Vec<T>) -> Vec<T> {
        assert_eq!(speed.len(), self.max_speed.len());

        let new_speed = speed
            .iter()
            .zip(self.max_speed.iter())
            .map(|(v, v_max)| {
                if v.abs() <= *v_max {
                    *v
                } else {
                    v_max.abs() * v.signum()
                }
            })
            .collect();

        new_speed
    }
}

/// The trait to restrict absolute speed
pub struct MaxSpeedAbs<T> {
    max_speed: T,
}

impl<T> MaxSpeedAbs<T> {
    pub fn new(max_speed: T) -> Self {
        Self { max_speed }
    }
}

impl<T: Float> PostSpeedCalc<T> for MaxSpeedAbs<T> {
    fn correct_speed(&mut self, speed: Vec<T>) -> Vec<T> {
        let current_speed_abs = speed
            .iter()
            .fold(T::zero(), |acc, vi| acc + (*vi) * (*vi))
            .sqrt();
        if current_speed_abs > self.max_speed {
            let new_speed = speed
                .iter()
                .map(|vi| (*vi) * self.max_speed / current_speed_abs)
                .collect();
            new_speed
        } else {
            speed
        }
    }
}

#[cfg(test)]
mod tests {
    use num::abs;
    use super::{MaxSpeedAbs, MaxSpeedDimensions};
    use crate::particleswarm::PostSpeedCalc;

    #[test]
    fn test_max_speed_dimensions_empty() {
        let max_speed: Vec<f32> = vec![];
        let speed: Vec<f32> = vec![];

        let mut post_speed = MaxSpeedDimensions::new(max_speed);
        let new_speed = post_speed.correct_speed(speed);

        assert_eq!(new_speed, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_max_speed_invalid_size() {
        let max_speed: Vec<f32> = vec![2.0_f32];
        let speed: Vec<f32> = vec![];

        let mut post_speed = MaxSpeedDimensions::new(max_speed);
        post_speed.correct_speed(speed);
    }

    #[test]
    fn test_max_speed_dimensions_not_change() {
        let max_speed: Vec<f32> = vec![10.0_f32, 10.0_f32];
        let speed: Vec<f32> = vec![5.0_f32, -5.0_f32];

        let mut post_speed = MaxSpeedDimensions::new(max_speed);
        let new_speed = post_speed.correct_speed(speed.clone());

        assert_eq!(new_speed, speed);
    }

    #[test]
    fn test_max_speed_dimensions_fix() {
        let max_speed: Vec<f32> = vec![10.0_f32, 10.0_f32, 10.0_f32];
        let speed: Vec<f32> = vec![15.0_f32, -15.0_f32, 5.0_f32];

        let mut post_speed = MaxSpeedDimensions::new(max_speed);
        let new_speed = post_speed.correct_speed(speed.clone());

        assert_eq!(new_speed, vec![10.0_f32, -10.0_f32, 5.0_f32]);
    }

    #[test]
    fn test_max_speed_abs_empty() {
        let max_speed = 10.0_f32;
        let speed: Vec<f32> = vec![];

        let mut post_speed = MaxSpeedAbs::new(max_speed);
        let new_speed = post_speed.correct_speed(speed);

        assert_eq!(new_speed, vec![]);
    }

    #[test]
    fn test_max_speed_abs_not_change() {
        let max_speed = 10.0_f32;
        let speed: Vec<f32> = vec![5.0_f32, -5.0_f32];

        let mut post_speed = MaxSpeedAbs::new(max_speed);
        let new_speed = post_speed.correct_speed(speed.clone());

        assert_eq!(new_speed, speed);
    }

    #[test]
    fn test_max_speed_abs_change() {
        let max_speed = 10.0_f32;
        let speed: Vec<f32> = vec![15.0_f32, -15.0_f32];

        let mut post_speed = MaxSpeedAbs::new(max_speed);
        let new_speed = post_speed.correct_speed(speed.clone());
        let new_speed_abs = (new_speed[0] * new_speed[0] + 
            new_speed[1] * new_speed[1]).sqrt();

        assert!(abs(new_speed_abs - max_speed) < 1e-3);
    }
}
