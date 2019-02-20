//! The module with functions to test optimization algorithms.

use num::Float;


/// Paraboloid.
///
/// y = (x0 - 0)^2 + (x1 - 1)^2 + (x2 - 2)^2 ... (xn - n)^2
/// The min val is 0.0 for point (0.0, 1.0, 2.0, ... n).
pub fn paraboloid<G: Float>(chromosomes: &Vec<G>) -> f64 {
    let mut result = G::from(0.0).unwrap();
    for (n, val) in chromosomes.iter().enumerate() {
        result = result + (*val - (G::from(n).unwrap() + G::from(1.0).unwrap())).powi(2);
    }

    result.to_f64().unwrap()
}
