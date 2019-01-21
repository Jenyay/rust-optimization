pub mod cross;
pub mod mutation;

use super::Optimizer;


pub struct Individual<T> {
    pub is_alive: bool,
    pub chromosomes: T,
}


impl <T> Individual<T> {
    pub fn new(chromosomes: T) -> Individual<T> {
        Individual {
            is_alive: true,
            chromosomes,
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


pub struct GeneticOptimizer<'a, T> {
    population_size: usize,
    goal: &'a Goal<T>,
    cross: &'a Cross<T>,
    mutation: &'a Mutation<T>,
    // pairing: &'a Fn(&Vec<Individual<T>>) -> Vec<Individual<'a, T>>,
    // selection: &'a Fn(&mut Vec<Individual<T>>),
}


impl <'a, T>GeneticOptimizer<'a, T> {
    pub fn new(population_size: usize,
               goal: &'a Goal<T>,
               cross: &'a Cross<T>,
               mutation: &'a Mutation<T>
               ) -> GeneticOptimizer<'a, T> {
        GeneticOptimizer {
            population_size,
            goal,
            cross,
            mutation,
        }
    }
}


impl <'a, T>Optimizer for GeneticOptimizer<'a, T> {
    fn run(&self) {
    }
}
