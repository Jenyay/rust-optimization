pub fn paraboloid(chromosomes: &Vec<f64>) -> f64 {
    let mut result = 0.0;
    for (n, val) in chromosomes.iter().enumerate() {
        result += (val - (n as f64 + 1.0)) * (val - (n as f64 + 1.0));
    }

    result
}
