pub fn paraboloid(chromosomes: &Vec<f64>) -> f64 {
    let mut result = 0.0;
    for val in chromosomes {
        result += val * val;
    }

    result
}
