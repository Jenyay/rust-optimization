# optlib-testfunc

[![Current Version](https://img.shields.io/crates/v/optlib_testfunc.svg)](https://crates.io/crates/optlib-testfunc)
[![Documentation](https://docs.rs/optlib-testfunc/badge.svg)](https://docs.rs/optlib-testfunc)
[![License](https://img.shields.io/crates/l/optlib_testfunc.svg)](https://crates.io/crates/optlib-testfunc)

The crate contains functions for optimization algorithms testing. All functions have one global minimum.

Optlib-testfunc contains in this version follow function:

* Paraboloid. y = (x0 - 1)^2 + (x1 - 2)^2 + (x2 - 3)^2 ... (xn - n)^2. For any x global minimum located in x' = (1.0, 2.0, ..., n). f(x') = 0.
* The Schwefel function. For any x lies in [-500.0; 500.0] global minimum located in x' = (420.9687, 420.9687, ...). f(x') = 0.
* The Rastrigin function. For any x lies in [-5.12; 5.12] global minimum located in x' = (0, 0, ...). f(x') = 0.
* The Rosenbrock function. For any x lies in [-inf; inf] global minimum located in x' = (1, 1, ...). f(x') = 0.
