pub mod cross;
pub mod mutation;

use std::f64;

use super::Optimizer;


pub struct Individual<T> {
    pub chromosomes: T,
    pub fitness: f64,
}


impl<T> Individual<T> {
    pub fn new(chromosomes: T) -> Individual<T> {
        Individual {
            chromosomes,
            fitness: f64::NAN,
        }
    }
}


pub trait Goal<T> {
    fn get(&mut self, chromosomes: &T) -> f64;
}


pub trait Creator<T> {
    fn create(&mut self) -> Vec<Individual<T>>;
}


pub trait Cross<T> {
    fn cross(&mut self, individuals: &Vec<Individual<T>>) -> Vec<Individual<T>>;
}


pub trait Mutation<T> {
    fn mutation(&mut self, individual: &mut Individual<T>);
}


pub trait Selection<T> {
    fn get_dead(&mut self, population: &Vec<Individual<T>>) -> Vec<usize>;
}


pub trait Pairing<T> {
    fn get_pairs(&mut self, population: &Vec<Individual<T>>) -> Vec<Vec<usize>>;
}


pub trait StopChecker<T> {
    fn finish(&mut self, population: &Vec<Individual<T>>) -> bool;
}


pub struct GeneticOptimizer<'a, T> {
    goal: &'a mut Goal<T>,
    creator: &'a mut Creator<T>,
    pairing: &'a mut Pairing<T>,
    cross: &'a mut Cross<T>,
    mutation: &'a mut Mutation<T>,
    selection: &'a mut Selection<T>,
    stop_checker: &'a mut StopChecker<T>,
}


impl <'a, T>GeneticOptimizer<'a, T> {
    pub fn new(goal: &'a mut Goal<T>,
               creator: &'a mut Creator<T>,
               pairing: &'a mut Pairing<T>,
               cross: &'a mut Cross<T>,
               mutation: &'a mut Mutation<T>,
               selection: &'a mut Selection<T>,
               stop_checker: &'a mut StopChecker<T>
               ) -> GeneticOptimizer<'a, T> {
        GeneticOptimizer {
            goal,
            creator,
            pairing,
            cross,
            mutation,
            selection,
            stop_checker,
        }
    }
}


impl<'a, T> Optimizer for GeneticOptimizer<'a, T> {
    fn run(&mut self) {
        let mut population = self.creator.create();
        while !self.stop_checker.finish(&population) {
        }
    }
}
