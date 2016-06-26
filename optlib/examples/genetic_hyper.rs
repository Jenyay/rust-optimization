extern crate optlib;

use optlib::genetic;


fn main() {

}


struct HyperSphereFitness {
    value: Option<f64>,
}

impl genetic::Fitness<f64> for HyperSphereFitness {
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
