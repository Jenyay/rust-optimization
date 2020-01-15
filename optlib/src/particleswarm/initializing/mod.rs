use num::{NumCast, Zero};

use crate::tools::RandomVectorCreator;
use crate::particleswarm::{CoordinatesInitializer, SpeedInitializer};

/// The struct to initialize particles coordinates with random value from given intervals.
pub struct RandomCoordinatesInitializer<T> {
    // Intervals for every dimension. Size of the vector must be equal to dimension.
    // The first value in tuple is minimum value, the second value is maximum value.
    intervals: Vec<(T, T)>,
    particles_count: usize,
    vector_creator: RandomVectorCreator,
}

impl<T> RandomCoordinatesInitializer<T> {
    /// Constructor.
    ///
    /// # Parameters
    /// `intervals` - vector of tuples. Size of the vector must be equal to dimension. The first value in tuple is minimum coordinate, the second value is maximum coordinate.
    /// `particles_count` - how many particles do you need to create.
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

/// The struct to initialze particles speed with random speed
pub struct RandomSpeedInitializer<T> {
    intervals: Vec<(T, T)>,
    particles_count: usize,
    vector_creator: RandomVectorCreator,
}

impl<T> RandomSpeedInitializer<T> {
    /// Constructor.
    ///
    /// # Parameters
    /// `intervals` - vector of tuples. Size of the vector must be equal to dimension. The first value in tuple is minimum speed, the second value is maximum seed.
    /// `particles_count` - how many particles do you need to create.
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

/// The struct to initialize particles speed with zeros.
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
