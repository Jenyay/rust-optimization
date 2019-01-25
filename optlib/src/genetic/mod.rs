pub mod cross;
pub mod mutation;

use std::f64;

use super::Optimizer;


#[derive(Debug, Copy)]
pub struct Individual<T: Clone> {
    pub chromosomes: T,
    pub fitness: f64,
}


impl<T: Clone> Individual<T> {
    pub fn new(chromosomes: T) -> Individual<T> {
        Individual {
            chromosomes,
            fitness: f64::NAN,
        }
    }
}


impl<T: Clone> Clone for Individual<T> {
    fn clone(&self) -> Individual<T> {
        Individual {
            chromosomes: self.chromosomes.clone(),
            fitness: self.fitness,
        }
    }
}


pub trait Goal<T: Clone> {
    fn get(&mut self, chromosomes: &T) -> f64;
}


pub trait Creator<T: Clone> {
    fn create(&mut self) -> Vec<Individual<T>>;
}


pub trait Cross<T: Clone> {
    fn cross(&mut self, individuals: &Vec<Individual<T>>) -> Vec<Individual<T>>;
}


pub trait Mutation<T: Clone> {
    fn mutation(&mut self, individual: &mut Individual<T>);
}


pub trait Selection<T: Clone> {
    fn get_dead(&mut self, population: &Vec<Individual<T>>) -> Vec<usize>;
}


pub trait Pairing<T: Clone> {
    fn get_pairs(&mut self, population: &Vec<Individual<T>>) -> Vec<Vec<usize>>;
}


pub trait StopChecker<T: Clone> {
    fn finish(&mut self, population: &Vec<Individual<T>>) -> bool;
}


pub struct GeneticOptimizer<'a, T: Clone> {
    goal: &'a mut Goal<T>,
    creator: &'a mut Creator<T>,
    pairing: &'a mut Pairing<T>,
    cross: &'a mut Cross<T>,
    mutation: &'a mut Mutation<T>,
    selection: &'a mut Selection<T>,
    stop_checker: &'a mut StopChecker<T>,

    best_individual: Option<Individual<T>>,
}


impl <'a, T: Clone>GeneticOptimizer<'a, T> {
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
            best_individual: None,
        }
    }
}


impl<'a, T: Clone> Optimizer<T> for GeneticOptimizer<'a, T> {
    fn find_min(&mut self) -> Option<T> {
        let mut population = self.creator.create();
        while !self.stop_checker.finish(&population) {
        }

        match self.best_individual.clone() {
            None => None,
            Some(individual) => Some(individual.chromosomes),
        }
    }
}
