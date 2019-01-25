pub mod cross;
pub mod mutation;

use std::f64;
use std::marker::PhantomData;

use super::Optimizer;

#[derive(Debug, Copy)]
pub struct Individual<T: Clone> {
    pub chromosomes: T,
    pub fitness: f64,
}

pub struct IndividualFactory<'a, T> {
    goal: &'a Goal<T>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Clone> IndividualFactory<'a, T> {
    pub fn new(goal: &'a Goal<T>) -> IndividualFactory<T> {
        IndividualFactory {
            goal,
            _phantom: PhantomData,
        }
    }

    pub fn create(&self, chromosomes: T) -> Individual<T> {
        let fitness = self.goal.get(&chromosomes);
        Individual {
            chromosomes,
            fitness,
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

pub trait Goal<T> {
    fn get(&self, chromosomes: &T) -> f64;
}

pub trait Creator<T: Clone> {
    fn create(&mut self, factory: &IndividualFactory<T>) -> Vec<Individual<T>>;
}

pub trait Cross<T: Clone> {
    fn cross(
        &self,
        individuals: &Vec<Individual<T>>,
        factory: &IndividualFactory<T>,
    ) -> Vec<Individual<T>>;
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
    creator: &'a mut Creator<T>,
    pairing: &'a mut Pairing<T>,
    cross: &'a mut Cross<T>,
    mutation: &'a mut Mutation<T>,
    selection: &'a mut Selection<T>,
    stop_checker: &'a mut StopChecker<T>,

    best_individual: Option<Individual<T>>,
    factory: IndividualFactory<'a, T>,
}

impl<'a, T: Clone> GeneticOptimizer<'a, T> {
    pub fn new(
        goal: &'a Goal<T>,
        creator: &'a mut Creator<T>,
        pairing: &'a mut Pairing<T>,
        cross: &'a mut Cross<T>,
        mutation: &'a mut Mutation<T>,
        selection: &'a mut Selection<T>,
        stop_checker: &'a mut StopChecker<T>,
    ) -> GeneticOptimizer<'a, T> {
        let factory = IndividualFactory::new(goal);
        GeneticOptimizer {
            creator,
            pairing,
            cross,
            mutation,
            selection,
            stop_checker,
            best_individual: None,
            factory,
        }
    }
}

impl<'a, T: Clone> Optimizer<T> for GeneticOptimizer<'a, T> {
    fn find_min(&mut self) -> Option<(T, f64)> {
        let mut population = self.creator.create(&self.factory);
        while !self.stop_checker.finish(&population) {
            // Pairing
            let mut new_individuals = self.run_pairing(&population);

            // Mutation
            self.run_mutation(&mut new_individuals);
            population.append(&mut new_individuals);

            // Selection
            self.run_selection(&mut population);

            // Find best
            self.best_individual = Some(self.find_best(&population));
        }

        match self.best_individual.clone() {
            None => None,
            Some(individual) => Some((individual.chromosomes, individual.fitness)),
        }
    }
}

impl<'a, T: Clone> GeneticOptimizer<'a, T> {
    fn run_pairing(&mut self, population: &Vec<Individual<T>>) -> Vec<Individual<T>> {
        let pairs: Vec<Vec<usize>> = self.pairing.get_pairs(&population);
        let mut new_individuals: Vec<Individual<T>> = Vec::with_capacity(pairs.len());

        for pair in pairs {
            let mut cross_individuals = Vec::with_capacity(pair.len());
            for i in pair {
                cross_individuals.push(population[i].clone());
            }

            let mut individuals = self.cross.cross(&cross_individuals, &self.factory);
            new_individuals.append(&mut individuals);
        }

        new_individuals
    }

    fn run_mutation(&mut self, individuals: &mut Vec<Individual<T>>) {
        for n in 0..individuals.len() {
            self.mutation.mutation(&mut individuals[n]);
        }
    }

    fn run_selection(&mut self, population: &mut Vec<Individual<T>>) {
        let mut dead_indexes = self.selection.get_dead(&population);
        dead_indexes.sort_unstable();
        for n in 0..dead_indexes.len() {
            population.remove(dead_indexes[n] - n);
        }
    }

    fn find_best(&self, population: &Vec<Individual<T>>) -> Individual<T> {
        let mut best_individual = population[0].clone();
        for n in 1..population.len() {
            if population[n].fitness < best_individual.fitness {
                best_individual = population[n].clone();
            }
        }

        best_individual
    }
}
