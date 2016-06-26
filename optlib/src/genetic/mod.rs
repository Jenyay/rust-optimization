extern crate rand;


/// Struct for single specimen
/// with type of chromosomes is `T`, type of fitness is `F`.
pub struct Specimen<T> {
    chromosomes: Vec<T>,
    fitness: f64,
}


impl<T> Specimen<T> {
    /// Create a new `Specimen<T, F>`.
    pub fn new(chromosomes: Vec<T>, fitness: f64) -> Specimen<T> {
        Specimen {
            chromosomes: chromosomes,
            fitness: fitness,
        }
    }
}


/// Fitness calculator for speciments.
pub trait Fitness<T> {
    fn calc(&mut self, chromo: Vec<T>) -> f64;
}


/// Mutation algorithm for speciments
pub trait MutationAlgorithm<T> {
    fn mutate(&self, chromo: Vec<T>) -> Vec<T>;
}



/// Cross algorithm for speciments
pub trait CrossAlgorithm<T> {
    fn cross(&self, parents: Vec<Specimen<T>>) -> Vec<Specimen<T>>;
}


/// Pairing algorithm for speciments.
pub trait PairingAlgorithm<T> {
    /// `candidates` - vertor of the speciments for pair selection.
    /// Return nested vector of the future parents.
    /// The first index - parents number,
    /// the internal vector store indexes of the speciments from `candidates`.
    fn pairing(&self, candidates: Vec<Specimen<T>>) -> Vec<Vec<i32>>;
}


/// Selection algorithm for speciments
pub trait SelectionAlgorithm<T> {
    /// Return indexes dead speciments.
    /// The dead speciments must be removed from population
    /// on the next iteration.
    fn get_dead(&self, population: Vec<Specimen<T>>) -> Vec<i32>;
}


/// Genetic algorithm realization
pub struct Genetic<T> {
    population: Vec<Specimen<T>>,
    mutation: Box<MutationAlgorithm<T>>,
    cross: Box<CrossAlgorithm<T>>,
    pairing: Box<PairingAlgorithm<T>>,
    selection: Box<SelectionAlgorithm<T>>,
}


impl<T> Genetic<T> {
    pub fn new(population: Vec<Specimen<T>>,
               mutation: Box<MutationAlgorithm<T>>,
               cross: Box<CrossAlgorithm<T>>,
               pairing: Box<PairingAlgorithm<T>>,
               selection: Box<SelectionAlgorithm<T>>) -> Genetic<T> {
        Genetic {
            population: population,
            mutation: mutation,
            cross: cross,
            pairing: pairing,
            selection: selection,
        }
    }
}
