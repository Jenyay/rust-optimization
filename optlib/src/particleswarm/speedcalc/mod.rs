use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use num::{Float, Num, NumCast};

use crate::particleswarm::{Particle, SpeedCalculator, Swarm};

/// ClassicSpeedCalculator implements the equation from the article
/// Kennedy, J.; Eberhart, R. (1995). "Particle Swarm Optimization".
/// Proceedings of IEEE International Conference on Neural Networks IV, pp.1942-1948.
pub struct ClassicSpeedCalculator<T> {
    phi_personal: T,
    phi_global: T,

    random: ThreadRng,
}

impl<T> ClassicSpeedCalculator<T> {
    pub fn new(phi_personal: T, phi_global: T) -> Self {
        Self {
            phi_personal,
            phi_global,
            random: rand::thread_rng(),
        }
    }
}

impl<T: NumCast + Num + Copy> SpeedCalculator<T> for ClassicSpeedCalculator<T> {
    fn calc_new_speed(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();
        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;

        let between = Uniform::new_inclusive(0.0_f32, 1.0_f32);
        let mut new_speed = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_personal = T::from(between.sample(&mut self.random)).unwrap();
            let r_global = T::from(between.sample(&mut self.random)).unwrap();

            let speed_item = particle.speed[i]
                + self.phi_personal
                    * r_personal
                    * (particle.best_personal_coordinates[i] - particle.coordinates[i])
                + self.phi_global * r_global * (global_best_solution[i] - particle.coordinates[i]);
            new_speed.push(speed_item);
        }

        new_speed
    }
}

/// CanonicalSpeedCalculator implements the "canonical" equation from the article
pub struct CanonicalSpeedCalculator<T> {
    phi_personal: T,
    phi_global: T,
    xi: T,

    random: ThreadRng,
}

impl<T: Float> CanonicalSpeedCalculator<T> {
    pub fn new(phi_personal: T, phi_global: T, k: T) -> Self {
        assert!(phi_personal + phi_global > T::from(4.0).unwrap());
        assert!(k > T::zero());
        assert!(k < T::one());

        let phi = phi_global + phi_personal;
        let xi = T::from(2.0).unwrap() * k
            / ((T::from(2.0).unwrap() - phi - (phi * phi - T::from(4.0).unwrap() * phi).sqrt())
                .abs());
        Self {
            phi_personal,
            phi_global,
            xi,
            random: rand::thread_rng(),
        }
    }
}

impl<T: NumCast + Num + Copy> SpeedCalculator<T> for CanonicalSpeedCalculator<T> {
    fn calc_new_speed(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();
        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;

        let between = Uniform::new_inclusive(0.0_f32, 1.0_f32);
        let mut new_speed = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_personal = T::from(between.sample(&mut self.random)).unwrap();
            let r_global = T::from(between.sample(&mut self.random)).unwrap();

            let speed_item = self.xi
                * (particle.speed[i]
                    + self.phi_personal * r_personal * (particle.best_personal_coordinates[i] - particle.coordinates[i])
                    + self.phi_global * r_global * (global_best_solution[i] - particle.coordinates[i]));
            new_speed.push(speed_item);
        }

        new_speed
    }
}
