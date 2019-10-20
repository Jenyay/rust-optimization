use std::cmp::Ordering;
use std::f64;

use super::{Agent, AlgorithmWithAgents, Goal, Optimizer};

/// Struct for single point (agent) in the search space
///
/// `T` - type of a point in the search space for goal function.
#[derive(Debug)]
pub struct Particle<T> {
    /// Point in the search space.
    coordinates: T,

    /// Speed of particle.
    speed: T,

    /// Value of function in the current coordinates.
    current_value: f64,

    /// The best coordinates for the particle.
    coordinates_best: T,

    /// Value of the function for the best coordinates for the particle.
    best_value: f64,
}

impl<T> Agent<T> for Particle<T> {
    fn get_goal(&self) -> f64 {
        self.current_value
    }

    fn get_parameter(&self) -> &T {
        &self.coordinates
    }
}

impl<T: Clone> Particle<T> {
    /// Return value of the goal function.
    pub fn new(coordinates: T, speed: T, current_value: f64) -> Self {
        let coordinates_best = coordinates.clone();
        let best_value = current_value;

        Self {
            coordinates,
            speed,
            current_value,
            coordinates_best,
            best_value,
        }
    }
}

/// The trait for logging for particle swarm algorithm.
///
/// `T` - type of a point in the search space for goal function.
pub trait Logger<T> {
    /// Will be called after swarm initializing.
    fn start(&mut self, _swarm: &Swarm<T>) {}

    /// Will be called before run algorithm (possibly after result algorithm after pause).
    fn resume(&mut self, _swarm: &Swarm<T>) {}

    /// Will be called in the end of iteration.
    fn next_iteration(&mut self, _swarm: &Swarm<T>) {}

    /// Will be called when algorithm will be stopped.
    fn finish(&mut self, _swarm: &Swarm<T>) {}
}

/// The trait to create initial particles swarm.
///
/// `T` - type of a point in the search space for goal function.
pub trait Creator<T> {
    /// Must return vector of the start points for a new particles.
    fn get_coordinates(&self) -> Vec<T>;

    /// Must return vector of speed for a new particles.
    fn get_speed(&self) -> Vec<T>;
}

/// The trait with break criterion for particle swarm algorithm.
///
/// `T` - type of a point in the search space for goal function.
pub trait StopChecker<T> {
    /// The method must return true if the algorithm must be stopped.
    fn can_stop(&mut self, swarm: &Swarm<T>) -> bool;
}

/// The trait may be used after moving the particle but before goal function calculating.
///
/// `T` - type of a point in the search space for goal function.
pub trait PostMove<T> {
    /// The method may modify chromosomes list before birth of the individuals.
    fn post_move(&mut self, swarm: &Swarm<T>);
}

/// Stores all particles.
///
/// `T` - type of a point in the search space for goal function.
pub struct Swarm<T> {
    particles: Vec<Particle<T>>,

    // Trait object for goal function.
    goal: Box<dyn Goal<T>>,

    // The best coordinates for current iteration.
    best_coordinates: T,

    // The best value for current iteration.
    best_value: f64,

    // Iteration number.
    iteration: usize,
}

impl<T: Clone> Swarm<T> {
    pub fn new(coordinates: Vec<T>, speed: Vec<T>, goal: Box<dyn Goal<T>>) -> Self {
        assert!(coordinates.len() == speed.len());
        let particles: Vec<Particle<T>> = coordinates
            .iter()
            .zip(speed.iter())
            .map(|cs| {
                let particle_coordinate = cs.0.clone();
                let particle_speed = cs.1.clone();
                let particle_value = goal.get(cs.0);
                Particle::new(particle_coordinate, particle_speed, particle_value)
            })
            .collect();

        let best_particle = Self::_find_best_particle(&particles);
        let best_coordinates = best_particle.coordinates.clone();
        let best_value = best_particle.best_value;

        let iteration = 0;

        Swarm {
            particles,
            goal,
            best_coordinates,
            best_value,
            iteration,
        }
    }

    fn _find_best_particle(particles: &Vec<Particle<T>>) -> &Particle<T> {
        assert!(particles.len() != 0);
        particles
            .iter()
            .min_by(|p1, p2| _compare_floats(p1.current_value, p2.current_value))
            .unwrap()
    }
}

pub struct ParticleSwarmOptimizer<T> {
    stop_checker: Box<dyn StopChecker<T>>,
    post_move: Vec<Box<dyn PostMove<T>>>,
    logger: Vec<Box<dyn Logger<T>>>,
    swarm: Swarm<T>,
}

impl<T: Clone> ParticleSwarmOptimizer<T> {
    pub fn new(
        goal: Box<dyn Goal<T>>,
        stop_checker: Box<dyn StopChecker<T>>,
        creator: Box<dyn Creator<T>>,
        post_move: Vec<Box<dyn PostMove<T>>>,
        logger: Vec<Box<dyn Logger<T>>>,
    ) -> Self {
        let mut coordinates = creator.get_coordinates();
        let mut speed = creator.get_speed();
        let mut swarm = Swarm::new(coordinates, speed, goal);

        ParticleSwarmOptimizer {
            stop_checker,
            post_move,
            logger,
            swarm,
        }
    }
}

fn _compare_floats(x: f64, y: f64) -> Ordering {
    if !x.is_finite() && !y.is_finite() {
        Ordering::Equal
    } else if x.is_finite() && !y.is_finite() {
        Ordering::Less
    } else if !x.is_finite() && y.is_finite() {
        Ordering::Greater
    } else {
        if x > y {
            Ordering::Greater
        } else if x < y {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[test]
fn test_compare_floats() {
    assert_eq!(
        _compare_floats(f64::INFINITY, f64::INFINITY),
        Ordering::Equal
    );
    assert_eq!(_compare_floats(f64::NAN, f64::NAN), Ordering::Equal);
    assert_eq!(_compare_floats(1.0_f64, f64::NAN), Ordering::Less);
    assert_eq!(_compare_floats(f64::NAN, 1.0_f64), Ordering::Greater);
    assert_eq!(_compare_floats(2.0_f64, 1.0_f64), Ordering::Greater);
    assert_eq!(_compare_floats(2.0_f64, 3.0_f64), Ordering::Less);
    assert_eq!(_compare_floats(3.0_f64, 3.0_f64), Ordering::Equal);
}
