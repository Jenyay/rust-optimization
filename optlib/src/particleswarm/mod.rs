use std::cmp::Ordering;
use std::f64;

use num::Float;

use super::tools::logging::Logger;
use super::{Agent, AgentsState, AlgorithmState, Goal, Optimizer};

/// The trait to create initial particles swarm.
///
/// `T` - type of a point in the search space for goal function.
pub trait Creator<T> {
    /// Must return vector of the start points for a new particles.
    fn get_coordinates(&self) -> Vec<Vec<T>>;

    /// Must return vector of speed for a new particles.
    fn get_speed(&self) -> Vec<Vec<T>>;
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

pub trait SpeedCalculator<T> {
    fn calc_new_speed(&self, swarm: &Swarm<T>, particle: &Particle<T>) -> Vec<T>;
}

/// Struct for single point (agent) in the search space
///
/// `T` - type of a point in the search space for goal function.
#[derive(Debug)]
pub struct Particle<T> {
    /// Point in the search space.
    coordinates: Vec<T>,

    /// Speed of particle.
    speed: Vec<T>,

    /// Value of function in the current coordinates.
    value: f64,

    /// Best coordinates for this particle
    best_personal_coordinates: Vec<T>,

    /// Best value for this particle
    best_personal_value: f64,
}

impl<T: Clone> Clone for Particle<T> {
    fn clone(&self) -> Self {
        let mut particle = Particle::new(self.coordinates.clone(), self.speed.clone(), self.value);
        particle.best_personal_coordinates = self.best_personal_coordinates.clone();
        particle.best_personal_value = self.best_personal_value;
        particle
    }
}

impl<T> Agent<Vec<T>> for Particle<T> {
    fn get_goal(&self) -> f64 {
        self.value
    }

    fn get_parameter(&self) -> &Vec<T> {
        &self.coordinates
    }
}

impl<T: Clone> Particle<T> {
    /// Return value of the goal function.
    fn new(coordinates: Vec<T>, speed: Vec<T>, value: f64) -> Self {
        let best_personal_coordinates = coordinates.clone();
        Self {
            coordinates,
            speed,
            value,
            best_personal_coordinates,
            best_personal_value: value,
        }
    }

    fn set_speed(&mut self, speed: Vec<T>) {
        self.speed = speed;
    }

    fn move_to(&mut self, new_coordinates: Vec<T>, value: f64) {
        self.coordinates = new_coordinates;
        self.value = value;

        if compare_floats(value, self.best_personal_value) == Ordering::Less {
            self.best_personal_coordinates = self.coordinates.clone();
            self.best_personal_value = value;
        }
    }
}

#[test]
fn test_particle_new() {
    let coordinates = vec![1.0_f32, 2.0_f32];
    let speed = vec![11.0_f32, 12.0_f32];
    let value = 21_f64;

    let particle = Particle::new(coordinates.clone(), speed.clone(), value);

    assert_eq!(particle.coordinates, coordinates);
    assert_eq!(particle.speed, speed);
    assert_eq!(particle.value, value);
    assert_eq!(particle.best_personal_coordinates, coordinates);
    assert_eq!(particle.best_personal_value, value);
}

#[test]
fn test_particle_move_to_better() {
    let coordinates = vec![1.0_f32, 2.0_f32];
    let speed = vec![11.0_f32, 12.0_f32];
    let value = 21_f64;

    let mut particle = Particle::new(coordinates.clone(), speed.clone(), value);

    let new_coordinates = vec![1.0_f32, 2.0_f32];
    let new_value = 10_f64;
    particle.move_to(new_coordinates.clone(), new_value);

    assert_eq!(particle.coordinates, new_coordinates);
    assert_eq!(particle.best_personal_coordinates, new_coordinates);
    assert_eq!(particle.best_personal_value, new_value);
}

#[test]
fn test_particle_move_to_worse() {
    let coordinates = vec![1.0_f32, 2.0_f32];
    let speed = vec![11.0_f32, 12.0_f32];
    let value = 20_f64;

    let mut particle = Particle::new(coordinates.clone(), speed.clone(), value);

    let new_coordinates = vec![1.0_f32, 2.0_f32];
    let new_value = 40_f64;
    particle.move_to(new_coordinates.clone(), new_value);

    assert_eq!(particle.coordinates, new_coordinates);
    assert_eq!(particle.best_personal_coordinates, coordinates);
    assert_eq!(particle.best_personal_value, value);
}

/// Stores all particles.
///
/// `T` - type of a point in the search space for goal function.
pub struct Swarm<T> {
    particles: Vec<Particle<T>>,

    /// The best coordinates for current iteration.
    best_particle: Option<Particle<T>>,

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

    /// Returns count of the particles in the swarm.
    pub fn len(&self) -> usize {
        self.particles.len()
    }

    /// Remove all particles and go to iteration 0.
    fn reset(&mut self) {
        self.particles.clear();
        self.best_particle = None;
        self.iteration = 0;
    }

    fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    fn replace_particles(&mut self, particles: Vec<Particle<T>>) {
        self.particles = particles;
        self.best_particle = Self::find_best_particle(&self.particles);
    }

    fn update_best_particle(&mut self) {
        self.best_particle = Self::find_best_particle(&self.particles);
    }

    fn find_best_particle(particles: &Vec<Particle<T>>) -> Option<Particle<T>> {
        if particles.is_empty() {
            None
        } else {
            let particle = particles
                .iter()
                .min_by(|p1, p2| compare_floats(p1.value, p2.value))
                .unwrap();
            Some(particle.clone())
        }
    }
}

#[test]
fn test_find_best_particle_empty() {
    let particles: Vec<Particle<f32>> = vec![];
    assert!(Swarm::find_best_particle(&particles).is_none());
}

#[test]
fn test_find_best_particle_single() {
    let particles: Vec<Particle<f32>> = vec![Particle::new(
        vec![1_f32, 2_f32],
        vec![10_f32, 20_f32],
        100_f64,
    )];
    let best_particle = Swarm::find_best_particle(&particles);
    assert!(best_particle.is_some());
}

#[test]
fn test_find_best_particle_many_01() {
    let particles: Vec<Particle<f32>> = vec![
        Particle::new(vec![1_f32, 2_f32], vec![10_f32, 20_f32], 100_f64),
        Particle::new(vec![3_f32, 4_f32], vec![10_f32, 20_f32], 50_f64),
    ];
    let best_particle = Swarm::find_best_particle(&particles);
    assert_eq!(best_particle.unwrap().value, 50_f64);
}

#[test]
fn test_find_best_particle_many_02() {
    let particles: Vec<Particle<f32>> = vec![
        Particle::new(vec![3_f32, 4_f32], vec![10_f32, 20_f32], 50_f64),
        Particle::new(vec![1_f32, 2_f32], vec![10_f32, 20_f32], 100_f64),
    ];
    let best_particle = Swarm::find_best_particle(&particles);
    assert_eq!(best_particle.unwrap().value, 50_f64);
}

pub struct ParticleSwarmOptimizer<T> {
    goal: Box<dyn Goal<Vec<T>>>,
    creator: Box<dyn Creator<T>>,
    stop_checker: Box<dyn StopChecker<T>>,
    speed_calculator: Box<dyn SpeedCalculator<T>>,
    post_move: Vec<Box<dyn PostMove<T>>>,
    loggers: Vec<Box<dyn Logger<Vec<T>>>>,
    swarm: Swarm<T>,
}

impl<T: Clone + Float> ParticleSwarmOptimizer<T> {
    pub fn new(
        goal: Box<dyn Goal<Vec<T>>>,
        stop_checker: Box<dyn StopChecker<T>>,
        creator: Box<dyn Creator<T>>,
        speed_calculator: Box<dyn SpeedCalculator<T>>,
        post_move: Vec<Box<dyn PostMove<T>>>,
        loggers: Vec<Box<dyn Logger<Vec<T>>>>,
    ) -> Self {
        let swarm = Swarm::new();

        ParticleSwarmOptimizer {
            goal,
            creator,
            stop_checker,
            speed_calculator,
            post_move,
            loggers,
            swarm,
        }
    }

    fn renew_swarm(&mut self) {
        let mut coordinates = self.creator.get_coordinates();
        let speed = self.creator.get_speed();
        assert!(coordinates.len() == speed.len());

        for mut current_coordinates in &mut coordinates {
            self.post_move
                .iter()
                .for_each(|post_move| post_move.post_move(&mut current_coordinates));
        }

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

        self.swarm.reset();
        self.swarm.replace_particles(particles);
    }

    /// Main algorithm steps is here
    pub fn next_iterations(&mut self) -> Option<(Vec<T>, f64)> {
        for logger in &mut self.loggers {
            logger.resume(&self.swarm);
        }

        while !self.stop_checker.can_stop(&self.swarm) {
            for n in 0..self.swarm.particles.len() {
                // Calculate new speed
                let new_speed = self
                    .speed_calculator
                    .calc_new_speed(&self.swarm, &self.swarm.particles[n]);
                self.swarm.particles[n].set_speed(new_speed);

                // Calculate new coordinates
                let mut new_coordinates: Vec<T> = self.swarm.particles[n]
                    .coordinates
                    .iter()
                    .zip(self.swarm.particles[n].speed.iter())
                    .map(|(coord, speed)| *coord + *speed)
                    .collect();

                // Correct coordinates
                self.post_move
                    .iter()
                    .for_each(|post_move| post_move.post_move(&mut new_coordinates));

                // Calculate new value for the particle
                let new_value = self.goal.get(&new_coordinates);

                self.swarm.particles[n].move_to(new_coordinates, new_value);
            }

            self.swarm.update_best_particle();
            self.swarm.next_iteration();
        }

        match &self.swarm.best_particle {
            None => None,
            Some(particle) => Some((particle.coordinates.clone(), particle.value)),
        }
    }
}

impl<T: Clone + Float> Optimizer<Vec<T>> for ParticleSwarmOptimizer<T> {
    fn find_min(&mut self) -> Option<(Vec<T>, f64)> {
        self.renew_swarm();

        for logger in &mut self.loggers {
            logger.start(&self.swarm);
        }

        self.next_iterations()
    }
}

impl<T: Clone> AlgorithmState<Vec<T>> for Swarm<T> {
    fn get_best_solution(&self) -> Option<(Vec<T>, f64)> {
        match &self.best_particle {
            None => None,
            Some(particle) => Some((particle.coordinates.clone(), particle.value)),
        }
    }

    fn get_iteration(&self) -> usize {
        self.iteration
    }
}

impl<T: Clone> AgentsState<Vec<T>> for Swarm<T> {
    type Agent = Particle<T>;

    /// Returns vector with references to all agents
    fn get_agents(&self) -> Vec<&Self::Agent> {
        let mut agents: Vec<&Self::Agent> = Vec::with_capacity(self.len());
        for particle in self.particles.iter() {
            agents.push(particle);
        }

        agents
    }
}

fn compare_floats(x: f64, y: f64) -> Ordering {
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
        compare_floats(f64::INFINITY, f64::INFINITY),
        Ordering::Equal
    );
    assert_eq!(compare_floats(f64::NAN, f64::NAN), Ordering::Equal);
    assert_eq!(compare_floats(1.0_f64, f64::NAN), Ordering::Less);
    assert_eq!(compare_floats(f64::NAN, 1.0_f64), Ordering::Greater);
    assert_eq!(compare_floats(2.0_f64, 1.0_f64), Ordering::Greater);
    assert_eq!(compare_floats(2.0_f64, 3.0_f64), Ordering::Less);
    assert_eq!(compare_floats(3.0_f64, 3.0_f64), Ordering::Equal);
}
