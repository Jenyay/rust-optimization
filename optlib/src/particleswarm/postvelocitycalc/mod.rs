use num::Float;

use crate::particleswarm::PostVelocityCalc;

/// The trait to restrict value for every dimension of velocity
pub struct MaxVelocityDimensions<T> {
    max_velocity: Vec<T>,
}

impl<T> MaxVelocityDimensions<T> {
    pub fn new(max_velocity: Vec<T>) -> Self {
        Self { max_velocity }
    }
}

impl<T: Float> PostVelocityCalc<T> for MaxVelocityDimensions<T> {
    fn correct_velocity(&mut self, velocity: Vec<T>) -> Vec<T> {
        assert_eq!(velocity.len(), self.max_velocity.len());

        let new_velocity = velocity
            .iter()
            .zip(self.max_velocity.iter())
            .map(|(v, v_max)| {
                if v.abs() <= *v_max {
                    *v
                } else {
                    v_max.abs() * v.signum()
                }
            })
            .collect();

        new_velocity
    }
}

/// The trait to restrict absolute velocity
pub struct MaxVelocityAbs<T> {
    max_velocity: T,
}

impl<T> MaxVelocityAbs<T> {
    pub fn new(max_velocity: T) -> Self {
        Self { max_velocity }
    }
}

impl<T: Float> PostVelocityCalc<T> for MaxVelocityAbs<T> {
    fn correct_velocity(&mut self, velocity: Vec<T>) -> Vec<T> {
        let current_velocity_abs = velocity
            .iter()
            .fold(T::zero(), |acc, vi| acc + (*vi) * (*vi))
            .sqrt();
        if current_velocity_abs > self.max_velocity {
            let new_velocity = velocity
                .iter()
                .map(|vi| (*vi) * self.max_velocity / current_velocity_abs)
                .collect();
            new_velocity
        } else {
            velocity
        }
    }
}

#[cfg(test)]
mod tests {
    use num::abs;
    use super::{MaxVelocityAbs, MaxVelocityDimensions};
    use crate::particleswarm::PostVelocityCalc;

    #[test]
    fn test_max_velocity_dimensions_empty() {
        let max_velocity: Vec<f32> = vec![];
        let velocity: Vec<f32> = vec![];

        let mut post_velocity = MaxVelocityDimensions::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity);

        assert_eq!(new_velocity, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_max_velocity_invalid_size() {
        let max_velocity: Vec<f32> = vec![2.0_f32];
        let velocity: Vec<f32> = vec![];

        let mut post_velocity = MaxVelocityDimensions::new(max_velocity);
        post_velocity.correct_velocity(velocity);
    }

    #[test]
    fn test_max_velocity_dimensions_not_change() {
        let max_velocity: Vec<f32> = vec![10.0_f32, 10.0_f32];
        let velocity: Vec<f32> = vec![5.0_f32, -5.0_f32];

        let mut post_velocity = MaxVelocityDimensions::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity.clone());

        assert_eq!(new_velocity, velocity);
    }

    #[test]
    fn test_max_velocity_dimensions_fix() {
        let max_velocity: Vec<f32> = vec![10.0_f32, 10.0_f32, 10.0_f32];
        let velocity: Vec<f32> = vec![15.0_f32, -15.0_f32, 5.0_f32];

        let mut post_velocity = MaxVelocityDimensions::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity.clone());

        assert_eq!(new_velocity, vec![10.0_f32, -10.0_f32, 5.0_f32]);
    }

    #[test]
    fn test_max_velocity_abs_empty() {
        let max_velocity = 10.0_f32;
        let velocity: Vec<f32> = vec![];

        let mut post_velocity = MaxVelocityAbs::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity);

        assert_eq!(new_velocity, vec![]);
    }

    #[test]
    fn test_max_velocity_abs_not_change() {
        let max_velocity = 10.0_f32;
        let velocity: Vec<f32> = vec![5.0_f32, -5.0_f32];

        let mut post_velocity = MaxVelocityAbs::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity.clone());

        assert_eq!(new_velocity, velocity);
    }

    #[test]
    fn test_max_velocity_abs_change() {
        let max_velocity = 10.0_f32;
        let velocity: Vec<f32> = vec![15.0_f32, -15.0_f32];

        let mut post_velocity = MaxVelocityAbs::new(max_velocity);
        let new_velocity = post_velocity.correct_velocity(velocity.clone());
        let new_velocity_abs = (new_velocity[0] * new_velocity[0] + 
            new_velocity[1] * new_velocity[1]).sqrt();

        assert!(abs(new_velocity_abs - max_velocity) < 1e-3);
    }
}
