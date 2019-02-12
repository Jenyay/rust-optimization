use num::Float;


pub fn paraboloid<G: Float>(chromosomes: &Vec<G>) -> f64 {
    let mut result = G::from(0.0).unwrap();
    for (n, val) in chromosomes.iter().enumerate() {
        result = result + (*val - (G::from(n).unwrap() + G::from(1.0).unwrap())).powi(2);
    }

    result.to_f64().unwrap()
}
