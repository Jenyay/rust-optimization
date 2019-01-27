pub mod cross;
pub mod mutation;

use std::f64;
use std::slice;
use std::ops;

use super::Optimizer;

#[derive(Debug)]
pub struct Individual<T: Clone> {
    chromosomes: T,
    fitness: f64,
    alive: bool,
}

impl<T: Clone> Clone for Individual<T> {
    fn clone(&self) -> Individual<T> {
        Individual {
            chromosomes: self.chromosomes.clone(),
            fitness: self.fitness,
            alive: self.alive,
        }
    }
}

impl<T: Clone> Individual<T> {
    pub fn get_chromosomes(&self) -> T {
        self.chromosomes.clone()
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}

pub struct Population<'a, T: Clone> {
    goal: &'a Goal<T>,
    individuals: Vec<Individual<T>>,
    best_individual: Option<Individual<T>>,
    iteration: usize,
}

impl<'a, T: Clone> Population<'a, T> {
    pub fn new(goal: &'a Goal<T>) -> Self {
        Population {
            goal,
            individuals: vec![],
            best_individual: None,
            iteration: 0,
        }
    }

    pub fn push(&mut self, chromosomes: T) {
        let fitness = self.goal.get(&chromosomes);
        let new_individual = Individual {
            chromosomes,
            fitness,
            alive: true,
        };
        match self.best_individual.clone() {
            None => self.best_individual = Some(new_individual.clone()),
            Some(old_best) => if new_individual.get_fitness() < old_best.get_fitness() {
                self.best_individual = Some(new_individual.clone());
            },
        }
        self.individuals.push(new_individual);
    }

    pub fn iter(&self) -> slice::Iter<Individual<T>> {
        self.individuals.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Individual<T>> {
        self.individuals.iter_mut()
    }

    pub fn get_iteration(&self) -> usize {
        self.iteration
    }
}


impl<'a, T: Clone> ops::Index<usize> for Population<'a, T> {
    type Output = Individual<T>;

    fn index(&self, index: usize) -> &Individual<T> {
        &self.individuals[index]
    }
}

pub trait Goal<T> {
    fn get(&self, chromosomes: &T) -> f64;
}

pub trait Creator<T: Clone> {
    fn create(&mut self) -> Vec<T>;
}

pub trait Cross<T: Clone> {
    fn cross(&self, individuals: &Vec<T>) -> Vec<T>;
}

pub trait Mutation<T: Clone> {
    fn mutation(&mut self, chromosomes: &mut T);
}

pub trait Selection<T: Clone> {
    fn kill(&mut self, population: &mut Population<T>);
}

pub trait Pairing<T: Clone> {
    fn get_pairs(&mut self, population: &Population<T>) -> Vec<Vec<usize>>;
}

pub trait StopChecker<T: Clone> {
    fn finish(&mut self, population: &Population<T>) -> bool;
}

pub struct GeneticOptimizer<'a, T: Clone> {
    goal: &'a Goal<T>,
    creator: &'a mut Creator<T>,
    pairing: &'a mut Pairing<T>,
    cross: &'a mut Cross<T>,
    mutation: &'a mut Mutation<T>,
    selection: &'a mut Selection<T>,
    stop_checker: &'a mut StopChecker<T>,
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

impl<'a, T: Clone> Optimizer<T> for GeneticOptimizer<'a, T> {
    fn find_min(&mut self) -> Option<(T, f64)> {
        let start_chromo = self.creator.create();
        let mut population = Population::new(self.goal);

        start_chromo.iter().for_each(|chromo| population.push(chromo.clone()));

        while !self.stop_checker.finish(&population) {
            // Pairing
            let mut children_chromo = self.run_pairing(&population);

            // Mutation
            children_chromo
                .iter_mut()
                .for_each(|chromo| self.mutation.mutation(chromo));

            // Add new individuals to population
            children_chromo.iter().for_each(|chromo| population.push(chromo.clone()));

            // Selection
            self.selection.kill(&mut population);
        }

        match population.best_individual.clone() {
            None => None,
            Some(individual) => Some((individual.chromosomes, individual.fitness)),
        }
    }
}

impl<'a, T: Clone> GeneticOptimizer<'a, T> {
    fn run_pairing(&mut self, population: &Population<T>) -> Vec<T> {
        let pairs: Vec<Vec<usize>> = self.pairing.get_pairs(&population);
        let mut new_chromosomes: Vec<T> = Vec::with_capacity(pairs.len());

        for pair in pairs {
            let mut cross_chromosomes = Vec::with_capacity(pair.len());
            for i in pair {
                cross_chromosomes.push(population[i].get_chromosomes());
            }

            let mut child_chromosomes = self.cross.cross(&cross_chromosomes);
            new_chromosomes.append(&mut child_chromosomes);
        }

        new_chromosomes
    }
}
