pub mod cross;
pub mod mutation;

use std::f64;

use super::Optimizer;


pub struct Individual<T> {
    pub chromosomes: T,
    pub fitness: f64,
}


impl <T> Individual<T> {
    pub fn new(chromosomes: T) -> Individual<T> {
        Individual {
            chromosomes,
            fitness: f64::NAN,
        }
    }
}


pub trait Goal<T> {
    fn get(&self, chromosomes: &T) -> f64;
}


pub trait Cross<T> {
    fn cross(&self, individuals: &Vec<Individual<T>>) -> Vec<Individual<T>>;
}


pub trait Mutation<T> {
    fn mutation(&self, individual: &mut Individual<T>);
}


pub trait Selection<T> {
    fn get_dead(&self, population: &Vec<Individual<T>>) -> Vec<usize>;
}


pub trait Pairing<T> {
    fn get_pairs(&self, population: &Vec<Individual<T>>) -> Vec<Vec<usize>>;
}


pub struct GeneticOptimizer<'a, T> {
    population_size: usize,
    goal: &'a Goal<T>,
    pairing: &'a Pairing<T>,
    cross: &'a Cross<T>,
    mutation: &'a Mutation<T>,
    selection: &'a Selection<T>,
}


impl <'a, T>GeneticOptimizer<'a, T> {
    pub fn new(population_size: usize,
               goal: &'a Goal<T>,
               pairing: &'a Pairing<T>,
               cross: &'a Cross<T>,
               mutation: &'a Mutation<T>,
               selection: &'a Selection<T>
               ) -> GeneticOptimizer<'a, T> {
        GeneticOptimizer {
            population_size,
            goal,
            pairing,
            cross,
            mutation,
            selection,
        }
    }
}


impl <'a, T>Optimizer for GeneticOptimizer<'a, T> {
    fn run(&self) {
    }
}
