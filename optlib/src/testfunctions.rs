//! The module with functions to test optimization algorithms.

use num::Float;


/// Paraboloid.
///
/// y = (x0 - 0)^2 + (x1 - 1)^2 + (x2 - 2)^2 ... (xn - n)^2
/// The min val is 0.0 for point (0.0, 1.0, 2.0, ... n).
pub fn paraboloid<G: Float>(x: &Vec<G>) -> f64 {
    let mut result = G::from(0.0).unwrap();
    for (n, val) in x.iter().enumerate() {
        result = result + (*val - (G::from(n).unwrap() + G::from(1.0).unwrap())).powi(2);
    }

    result.to_f64().unwrap()
}

/// Schwefel function
///
/// # Parameters
/// Any x lies in [-500.0; 500.0]. 
/// Global minimum is x' = (420.9687, 420.9687, ...).
/// f(x') = 0
pub fn schwefel<G: Float>(x: &Vec<G>) -> f64 {
    let result = G::from(418.9829).unwrap() * G::from(x.len()).unwrap() - x.iter().fold(G::zero(), |acc, &xi| acc + xi * xi.abs().sqrt().sin());

    result.to_f64().unwrap()
}

#[cfg(test)]
mod functions_test
{
    use super::*;

    #[test]
    pub fn schwefel_minimum() {
        let x = vec![420.9687, 420.9687, 420.9687, 420.9687];
        let schwefel_value = schwefel(&x);
        assert!(schwefel_value < 1e-4);
    }
}
