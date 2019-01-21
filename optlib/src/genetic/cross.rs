use num::Float;


pub fn cross_middle<T>(chromosomes: &Vec<T>) -> T
    where T: Float
{
    assert!(chromosomes.len() >= 2);
    let mut result = chromosomes[0].clone();
    for n in 1..chromosomes.len() {
        result = result + chromosomes[n].clone();
    }

    result = result / T::from(chromosomes.len()).unwrap();
    result
}
