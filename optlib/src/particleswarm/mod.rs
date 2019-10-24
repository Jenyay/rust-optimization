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
    value: f64,
}

impl<T: Clone> Clone for Particle<T> {
    fn clone(&self) -> Self {
        Particle::new(self.coordinates.clone(), self.speed.clone(), self.value)
    }
}

impl<T> Agent<T> for Particle<T> {
    fn get_goal(&self) -> f64 {
        self.value
    }

    fn get_parameter(&self) -> &T {
        &self.coordinates
    }
}

impl<T: Clone> Particle<T> {
    /// Return value of the goal function.
    pub fn new(coordinates: T, speed: T, value: f64) -> Self {
        Self {
            coordinates,
            speed,
            value,
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
    /// The method may modify coordinates list before calculate goal function
    fn post_move(&self, coordinates: &mut Vec<T>);
}

/// Stores all particles.
///
/// `T` - type of a point in the search space for goal function.
pub struct Swarm<T> {
    particles: Vec<Particle<T>>,

    // Trait object for goal function.
    // goal: Box<dyn Goal<T>>,

    // The best coordinates for current iteration.
    best_particle: Option<Particle<T>>,

    // The best value for current iteration.
    // best_value: f64,

    // Iteration number.
    iteration: usize,
}

impl<T: Clone> Swarm<T> {
    pub fn new() -> Self {
        Swarm {
            particles: vec![],
            best_particle: None,
            iteration: 0,
        }
    }

    /// Remove all particles and go to iteration 0.
    fn reset(&mut self) {
        self.particles.clear();
        self.best_particle = None;
        self.iteration = 0;
    }

    fn update(&mut self, particles: Vec<Particle<T>>) {
        self.particles = particles;
        self.best_particle = Self::_find_best_particle(&self.particles);
    }

    fn _find_best_particle(particles: &Vec<Particle<T>>) -> Option<Particle<T>> {
        if particles.is_empty() {
            None
        } else {
            let particle = particles
                .iter()
                .min_by(|p1, p2| Self::_compare_floats(p1.value, p2.value))
                .unwrap();
            Some(particle.clone())
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
}

pub struct ParticleSwarmOptimizer<T> {
    goal: Box<dyn Goal<T>>,
    creator: Box<dyn Creator<T>>,
    stop_checker: Box<dyn StopChecker<T>>,
    post_move: Vec<Box<dyn PostMove<T>>>,
    loggers: Vec<Box<dyn Logger<T>>>,
    swarm: Swarm<T>,
}

impl<T: Clone> ParticleSwarmOptimizer<T> {
    pub fn new(
        goal: Box<dyn Goal<T>>,
        stop_checker: Box<dyn StopChecker<T>>,
        creator: Box<dyn Creator<T>>,
        post_move: Vec<Box<dyn PostMove<T>>>,
        loggers: Vec<Box<dyn Logger<T>>>,
    ) -> Self {
        let swarm = Swarm::new();

        ParticleSwarmOptimizer {
            goal,
            creator,
            stop_checker,
            post_move,
            loggers,
            swarm,
        }
    }

    fn update_swarm(&mut self, coordinates: &mut Vec<T>, speed: Vec<T>) {
        assert!(coordinates.len() == speed.len());

        self.post_move
            .iter()
            .for_each(|post_move| post_move.post_move(coordinates));

        let particles: Vec<Particle<T>> = coordinates
            .iter()
            .zip(speed.iter())
            .map(|cs| {
                let particle_coordinate = cs.0.clone();
                let particle_speed = cs.1.clone();
                let particle_value = self.goal.get(cs.0);
                Particle::new(particle_coordinate, particle_speed, particle_value)
            })
            .collect();
        self.swarm.update(particles);
    }

    pub fn next_iterations(&mut self) -> Option<(T, f64)> {
        for logger in &mut self.loggers {
            logger.resume(&self.swarm);
        }

        while !self.stop_checker.can_stop(&self.swarm) {
        }

        match &self.swarm.best_particle {
            None => None,
            Some(particle) => Some((particle.coordinates.clone(), particle.value))
        }
    }
}

impl<T: Clone> Optimizer<T> for ParticleSwarmOptimizer<T> {
    fn find_min(&mut self) -> Option<(T, f64)> {
        self.swarm.reset();

        let mut coordinates = self.creator.get_coordinates();
        let speed = self.creator.get_speed();
        self.update_swarm(&mut coordinates, speed);

        for logger in &mut self.loggers {
            logger.start(&self.swarm);
        }

        self.next_iterations()
    }
}

#[test]
fn test_compare_floats() {
    assert_eq!(
        Swarm::_compare_floats(f64::INFINITY, f64::INFINITY),
        Ordering::Equal
    );
    assert_eq!(Swarm::_compare_floats(f64::NAN, f64::NAN), Ordering::Equal);
    assert_eq!(Swarm::_compare_floats(1.0_f64, f64::NAN), Ordering::Less);
    assert_eq!(Swarm::_compare_floats(f64::NAN, 1.0_f64), Ordering::Greater);
    assert_eq!(Swarm::_compare_floats(2.0_f64, 1.0_f64), Ordering::Greater);
    assert_eq!(Swarm::_compare_floats(2.0_f64, 3.0_f64), Ordering::Less);
    assert_eq!(Swarm::_compare_floats(3.0_f64, 3.0_f64), Ordering::Equal);
}
