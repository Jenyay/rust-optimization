use num::{NumCast, Zero};

use crate::tools::RandomVectorCreator;
use crate::particleswarm::{CoordinatesInitializer, SpeedInitializer};

pub struct RandomCoordinatesInitializer<T> {
    intervals: Vec<(T, T)>,
    particles_count: usize,
    vector_creator: RandomVectorCreator,
}

impl<T> RandomCoordinatesInitializer<T> {
    pub fn new(intervals: Vec<(T, T)>, particles_count: usize) -> Self {
        Self {
            intervals,
            particles_count,
            vector_creator: RandomVectorCreator::new(),
        }
    }
}

impl<T: NumCast + PartialOrd> CoordinatesInitializer<T> for RandomCoordinatesInitializer<T> {
    fn get_coordinates(&mut self) -> Vec<Vec<T>> {
        (0..self.particles_count)
            .map(|_| self.vector_creator.create_vec(&self.intervals))
            .collect()
    }
}

pub struct RandomSpeedInitializer<T> {
    intervals: Vec<(T, T)>,
    particles_count: usize,
    vector_creator: RandomVectorCreator,
}

impl<T> RandomSpeedInitializer<T> {
    pub fn new(intervals: Vec<(T, T)>, particles_count: usize) -> Self {
        Self {
            intervals,
            particles_count,
            vector_creator: RandomVectorCreator::new(),
        }
    }
}

impl<T: NumCast + PartialOrd> SpeedInitializer<T> for RandomSpeedInitializer<T> {
    fn get_speed(&mut self) -> Vec<Vec<T>> {
        (0..self.particles_count)
            .map(|_| self.vector_creator.create_vec(&self.intervals))
            .collect()
    }
}

pub struct ZeroSpeedInitializer {
    dimension: usize,
    particles_count: usize,
}

impl ZeroSpeedInitializer {
    pub fn new(dimension: usize, particles_count: usize) -> Self {
        Self { dimension, particles_count }
    }
}

impl<T: NumCast + PartialOrd + Zero + Clone> SpeedInitializer<T> for ZeroSpeedInitializer {
    fn get_speed(&mut self) -> Vec<Vec<T>> {
        (0..self.particles_count)
            .map(|_| vec![T::zero(); self.dimension])
            .collect()
    }
}
