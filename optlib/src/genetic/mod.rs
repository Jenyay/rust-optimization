struct Specimen<T, F: Fitness<T>> {
    chromosomes: Vec<T>,
    fitness: F,
}


impl <T, F: Fitness<T>> Specimen<T, F> {
    pub fn new(chromosomes: Vec<T>, fitness: F) -> Specimen<T, F> {
        Specimen {
            chromosomes: chromosomes,
            fitness: fitness,
        }
    }
}


trait Fitness<T> {
    fn calc(&mut self, chromo: Vec<T>) -> f64;
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
