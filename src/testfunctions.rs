//! The module with functions to test optimization algorithms.

use num::Float;


/// Paraboloid.
///
/// y = (x0 - 1)^2 + (x1 - 2)^2 + (x2 - 3)^2 ... (xn - n)^2
/// The min val is 0.0 for point (0.0, 1.0, 2.0, ... n).
///
/// ```
/// use optlib::testfunctions::paraboloid;
///
/// let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let value = paraboloid(&x);
///
/// assert!(value < 1e-5);
/// assert!(value >= 0.0);
/// ```
pub fn paraboloid<G: Float>(x: &Vec<G>) -> f64 {
    let mut result = G::from(0.0).unwrap();
    for (n, val) in x.iter().enumerate() {
        result = result + (*val - (G::from(n).unwrap() + G::from(1.0).unwrap())).powi(2);
    }

    result.to_f64().unwrap()
}

/// The Schwefel function
///
/// # Parameters
/// Any x lies in [-500.0; 500.0]. 
/// Global minimum is x' = (420.9687, 420.9687, ...).
/// f(x') = 0
///
/// ```
/// use optlib::testfunctions::schwefel;
///
/// let x = vec![420.9687, 420.9687, 420.9687, 420.9687];
/// let value = schwefel(&x);
/// assert!(value.abs() < 1e-4);
/// ```
pub fn schwefel<G: Float>(x: &Vec<G>) -> f64 {
    let result = G::from(418.9829).unwrap() * G::from(x.len()).unwrap() - x.iter().fold(G::zero(), |acc, &xi| acc + xi * xi.abs().sqrt().sin());

    result.to_f64().unwrap()
}

/// The Rastrigin function
///
/// # Parameters
/// Global minimum is x' = (0, 0, ...) for xn in (-5.12; +5.12)
/// f(x') = 0
///
/// ```
/// use optlib::testfunctions::rastrigin;
///
/// let x = vec![0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32];
/// let value = rastrigin(&x);
/// assert!(value.abs() < 1e-7);
/// ```
pub fn rastrigin<G: Float>(x: &Vec<G>) -> f64 {
    let a = G::from(10.0_f64).unwrap();
    let pi = G::from(3.14159265358979_f64).unwrap();
    let result = a * G::from(x.len()).unwrap() +
        x.iter().fold(G::zero(), |acc, &xi| acc + xi * xi - a * (G::from(2).unwrap() * pi * xi).cos());

    result.to_f64().unwrap()
}

/// The Rosenbrock function
///
/// # Parameters
/// Global minimum is x' = (1, 1, ...) for xn in (-inf; +inf)
/// f(x') = 0
///
/// ```
/// use optlib::testfunctions::rosenbrock;
///
/// let x = vec![1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32];
/// let value = rosenbrock(&x);
/// assert!(value.abs() < 1e-7);
/// ```
pub fn rosenbrock<G: Float>(x: &Vec<G>) -> f64 {
    let mut sum = G::from(0.0).unwrap();
    for n in 0..x.len() - 1 {
        sum = sum + G::from(100.0).unwrap() * ((x[n + 1] - x[n] * x[n]).powi(2)) + (G::from(1.0).unwrap() - x[n]).powi(2);
    }

    sum.to_f64().unwrap()
}
