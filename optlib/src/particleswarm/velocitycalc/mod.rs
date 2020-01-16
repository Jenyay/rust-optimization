use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use num::{Float, Num, NumCast};

use crate::particleswarm::{Particle, VelocityCalculator, Swarm};

/// ClassicVelocityCalculator implements the equation from the article
/// Kennedy, J.; Eberhart, R. (1995). "Particle Swarm Optimization".
/// Proceedings of IEEE International Conference on Neural Networks IV, pp.1942-1948.
/// v_i = v_i + phi_p * r_p * (p_i - x_i) + phi_g * r_g * (g_i - x_i)
/// `v_i` - velocity projection for dimension i,
/// `p_i` - personal best coordinate,
/// `g_i` - global best coordinate,
/// `x_i` - current coordinate,
/// `phi_p`, `phi_g` - parameters,
/// `r_p`, `r_g` - random values in (0, 1)
pub struct ClassicVelocityCalculator<T> {
    phi_personal: T,
    phi_global: T,

    random: ThreadRng,
}

impl<T> ClassicVelocityCalculator<T> {
    pub fn new(phi_personal: T, phi_global: T) -> Self {
        Self {
            phi_personal,
            phi_global,
            random: rand::thread_rng(),
        }
    }
}

impl<T: NumCast + Num + Copy> VelocityCalculator<T> for ClassicVelocityCalculator<T> {
    fn calc_new_velocity(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();
        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;

        let between = Uniform::new_inclusive(0.0_f32, 1.0_f32);
        let mut new_velocity = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_personal = T::from(between.sample(&mut self.random)).unwrap();
            let r_global = T::from(between.sample(&mut self.random)).unwrap();

            let velocity_item = particle.velocity[i]
                + self.phi_personal
                    * r_personal
                    * (particle.best_personal_coordinates[i] - particle.coordinates[i])
                + self.phi_global * r_global * (global_best_solution[i] - particle.coordinates[i]);
            new_velocity.push(velocity_item);
        }

        new_velocity
    }
}

/// CanonicalVelocityCalculator implements the "canonical" equation:
/// v_i = xi * (v_i + phi_p * r_p * (p_i - x_i) + phi_g * r_g * (g_i - x_i))
/// `v_i` - velocity projection for dimension i,
/// `p_i` - personal best coordinate,
/// `g_i` - global best coordinate,
/// `x_i` - current coordinate,
/// `phi_p`, `phi_g` - parameters,
/// `r_p`, `r_g` - random values in (0, 1),
/// `xi` = 2 * alpha / (phi - 2),
/// `phi` = phi_p + phi_g
/// `alpha` in (0, 1),
/// `phi` must be greater than 4
pub struct CanonicalVelocityCalculator<T> {
    phi_personal: T,
    phi_global: T,
    xi: T,

    random: ThreadRng,
}

impl<T: Float> CanonicalVelocityCalculator<T> {
    pub fn new(phi_personal: T, phi_global: T, alpha: T) -> Self {
        assert!(phi_personal + phi_global > T::from(4.0).unwrap());
        assert!(alpha > T::zero());
        assert!(alpha < T::one());

        let phi = phi_global + phi_personal;
        let xi = T::from(2.0).unwrap() * alpha / (phi - T::from(2.0).unwrap());
        Self {
            phi_personal,
            phi_global,
            xi,
            random: rand::thread_rng(),
        }
    }
}

impl<T: NumCast + Num + Copy> VelocityCalculator<T> for CanonicalVelocityCalculator<T> {
    fn calc_new_velocity(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();
        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;

        let between = Uniform::new_inclusive(0.0_f32, 1.0_f32);
        let mut new_velocity = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_personal = T::from(between.sample(&mut self.random)).unwrap();
            let r_global = T::from(between.sample(&mut self.random)).unwrap();

            let velocity_item = self.xi
                * (particle.velocity[i]
                    + self.phi_personal
                        * r_personal
                        * (particle.best_personal_coordinates[i] - particle.coordinates[i])
                    + self.phi_global
                        * r_global
                        * (global_best_solution[i] - particle.coordinates[i]));
            new_velocity.push(velocity_item);
        }

        new_velocity
    }
}

/// Velocity update with negative reinforcement, global and current best and worst positions.
/// v_i = xi * (v_i
///     + phi_best_personal * rb_p * (p_best_i - x_i)
///     + phi_best_current * rb_c * (c_best_i - x_i)
///     + phi_best_global * rb_g * (g_best_i - x_i)
///     - phi_worst_personal * rw_p * (p_worst_i - x_i)
///     - phi_worst_current * rw_c * (c_worst_i - x_i)
///     - phi_worst_global * rw_g * (g_worst_i - x_i))
///
/// `v_i` - velocity projection for dimension i,
/// `p_best_i` - personal best coordinate,
/// `c_best_i` - best coordinate for current swarm,
/// `g_best_i` - global best coordinate,
/// `p_worst_i` - personal worst coordinate,
/// `c_worst_i` - worst coordinate for current swarm,
/// `g_worst_i` - global worst coordinate,
/// `xi` - parameter,
/// `x_i` - current coordinate,
/// `phi_best_personal`, `phi_best_current`, `phi_best_global`, `phi_worst_personal`, `phi_worst_current`, `phi_worst_global` - parameters,
/// `rb_p`, `rb_c`, `rb_g`, `rw_p`, `rw_c`, `rw_g` - random values in (0, 1),
pub struct NegativeReinforcement<T> {
    phi_best_personal: T,
    phi_best_current: T,
    phi_best_global: T,

    phi_worst_personal: T,
    phi_worst_current: T,
    phi_worst_global: T,

    xi: T,

    random: ThreadRng,
}
impl<T> NegativeReinforcement<T> {
    pub fn new(
        phi_best_personal: T,
        phi_best_current: T,
        phi_best_global: T,
        phi_worst_personal: T,
        phi_worst_current: T,
        phi_worst_global: T,
        xi: T,
    ) -> Self {
        Self {
            phi_best_personal,
            phi_best_current,
            phi_best_global,
            phi_worst_personal,
            phi_worst_current,
            phi_worst_global,
            xi,
            random: rand::thread_rng(),
        }
    }
}

impl<T: NumCast + Num + Copy> VelocityCalculator<T> for NegativeReinforcement<T> {
    fn calc_new_velocity(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();

        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;

        let current_best_particle = swarm.get_current_best_particle().unwrap();
        let current_best_solution = current_best_particle.coordinates;

        let global_worst_particle = swarm.worst_particle.as_ref().unwrap();
        let global_worst_solution = &global_worst_particle.coordinates;

        let current_worst_particle = swarm.get_current_worst_particle().unwrap();
        let current_worst_solution = current_worst_particle.coordinates;

        let between = Uniform::new_inclusive(0.0, 1.0);
        let mut new_velocity = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_best_global = T::from(between.sample(&mut self.random)).unwrap();
            let r_best_current = T::from(between.sample(&mut self.random)).unwrap();
            let r_best_personal = T::from(between.sample(&mut self.random)).unwrap();

            let r_worst_global = T::from(between.sample(&mut self.random)).unwrap();
            let r_worst_current = T::from(between.sample(&mut self.random)).unwrap();
            let r_worst_personal = T::from(between.sample(&mut self.random)).unwrap();

            let v_best_personal = self.phi_best_personal
                * r_best_personal
                * (particle.best_personal_coordinates[i] - particle.coordinates[i]);
            let v_best_current = self.phi_best_current
                * r_best_current
                * (current_best_solution[i] - particle.coordinates[i]);
            let v_best_global = self.phi_best_global
                * r_best_global
                * (global_best_solution[i] - particle.coordinates[i]);

            let v_worst_personal = self.phi_worst_personal
                * r_worst_personal
                * (particle.worst_personal_coordinates[i] - particle.coordinates[i]);
            let v_worst_current = self.phi_worst_current
                * r_worst_current
                * (current_worst_solution[i] - particle.coordinates[i]);
            let v_worst_global = self.phi_worst_global
                * r_worst_global
                * (global_worst_solution[i] - particle.coordinates[i]);

            let velocity_item = self.xi
                * (particle.velocity[i] + v_best_personal + v_best_current + v_best_global
                    - v_worst_personal
                    - v_worst_current
                    - v_worst_global);
            new_velocity.push(velocity_item);
        }

        new_velocity
    }
}

/// The trait to calculate the inertia coefficient (w) for InertiaVelocityCalculator
pub trait Inertia<T> {
    fn get(&mut self, iteration: usize) -> T;
}


/// The inertia coefficient (w) does not depend on the iteration number
pub struct ConstInertia<T> {
    w: T,
}

impl<T> ConstInertia<T> {
    pub fn new(w: T) -> Self {
        Self { w }
    }
}

impl<T: Clone> Inertia<T> for ConstInertia<T> {
    fn get(&mut self, _iteration: usize) -> T {
        self.w.clone()
    }
}

/// The inertia coefficient decreases linearly from w_max to w_min
pub struct LinearInertia<T> {
    w_min: T,
    w_max: T,
    t_max: usize,
}

impl<T: Float> LinearInertia<T> {
    pub fn new(w_min: T, w_max: T, t_max: usize) -> Self {
        Self {
            w_min,
            w_max,
            t_max,
        }
    }
}

impl<T: Float> Inertia<T> for LinearInertia<T> {
    fn get(&mut self, iteration: usize) -> T {
        self.w_max
            - (self.w_max - self.w_min) * T::from(iteration).unwrap() / T::from(self.t_max).unwrap()
    }
}

/// InertiaVelocityCalculator implements the equation with itertia coefficient w(t)
/// v_i = w(t) * v_i + phi_personal * r_p * (p_i - x_i) + phi_global * r_g * (g_i - x_i)
/// `v_i` - velocity projection for dimension i,
/// `p_i` - personal best coordinate,
/// `g_i` - global best coordinate,
/// `x_i` - current coordinate,
/// `phi_personal`, `phi_global` - parameters,
/// `r_p`, `r_g` - random values in (0, 1),
/// `w(t)` calculate with the `Inertia` trait,
/// `t` - iteration number,
pub struct InertiaVelocityCalculator<'a, T> {
    phi_personal: T,
    phi_global: T,
    inertia: Box<dyn Inertia<T> + 'a>,

    random: ThreadRng,
}

impl<'a, T> InertiaVelocityCalculator<'a, T> {
    pub fn new(phi_personal: T, phi_global: T, inertia: Box<dyn Inertia<T> + 'a>) -> Self {
        Self {
            phi_personal,
            phi_global,
            inertia,
            random: rand::thread_rng(),
        }
    }
}

impl<'a, T: NumCast + Num + Copy> VelocityCalculator<T> for InertiaVelocityCalculator<'a, T> {
    fn calc_new_velocity(&mut self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T> {
        let dimension = particle.coordinates.len();
        let global_best_particle = swarm.best_particle.as_ref().unwrap();
        let global_best_solution = &global_best_particle.coordinates;
        let inertia_ratio = self.inertia.get(swarm.iteration);

        let between = Uniform::new_inclusive(0.0_f32, 1.0_f32);
        let mut new_velocity = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let r_personal = T::from(between.sample(&mut self.random)).unwrap();
            let r_global = T::from(between.sample(&mut self.random)).unwrap();

            let velocity_item = inertia_ratio * particle.velocity[i]
                + self.phi_personal
                    * r_personal
                    * (particle.best_personal_coordinates[i] - particle.coordinates[i])
                + self.phi_global * r_global * (global_best_solution[i] - particle.coordinates[i]);
            new_velocity.push(velocity_item);
        }

        new_velocity
    }
}
