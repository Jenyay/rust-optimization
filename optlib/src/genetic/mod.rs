/// Struct for single specimen
/// with type of chromosomes is `T`, type of fitness is `F`.
struct Specimen<T, F: Fitness<T>> {
    chromosomes: Vec<T>,
    fitness: F,
}


impl <T, F: Fitness<T>> Specimen<T, F> {
    /// Create a new `Specimen<T, F>`.
    pub fn new(chromosomes: Vec<T>, fitness: F) -> Specimen<T, F> {
        Specimen {
            chromosomes: chromosomes,
            fitness: fitness,
        }
    }
}


/// Fitness calculator for speciments.
trait Fitness<T> {
    fn calc(&mut self, chromo: Vec<T>) -> f64;
}


/// Mutation algorithm for speciments
trait MutationAlgorithm<T> {
    fn mutate(chromo: Vec<T>) -> Vec<T>;
}



/// Cross algorithm for speciments
trait CrossAlgorithm<T, F: Fitness<T>> {
    fn cross(parents: Vec<Specimen<T, F>>) -> Vec<Specimen<T, F>>;
}


/// Pairing algorithm for speciments.
trait PairingAlgorithm<T, F: Fitness<T>> {
    /// `candidates` - vertor of the speciments for pair selection.
    /// Return nested vector of the future parents.
    /// The first index - parents number,
    /// the internal vector store indexes of the speciments from `candidates`.
    fn pairing(candidates: Vec<Specimen<T, F>>) -> Vec<Vec<i32>>;
}


/// Selection algorithm for speciments
trait SelectionAlgorithm<T, F: Fitness<T>> {
    /// Return indexes dead speciments.
    /// The dead speciments must be removed from population
    /// on the next iteration.
    fn get_dead(population: Vec<Specimen<T, F>>) -> Vec<i32>;
}


struct HyperSphereFitness {
    value: Option<f64>,
}

impl Fitness<f64> for HyperSphereFitness {
    fn calc(&mut self, chromo: Vec<f64>) -> f64 {
        match self.value {
            None => {
                let new_value = chromo.iter().fold(0.0, |sum, &x| sum + x * x);
                self.value = Some(new_value);
                new_value
            },
            Some(x) => x,
        }
    }
}
