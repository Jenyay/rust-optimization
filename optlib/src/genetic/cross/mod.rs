//! The module with most usable algorithms of crossing for various types.
//! The module contains struct which implements the `Cross` trait and functions to cross
//! chromosomes various types.

use std::mem;

use super::Cross;
use num::{Float, Num, NumCast};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Struct to cross all genes (`G` - type of genes) in chromosome of type Vec<G>.
pub struct VecCrossAllGenes<G> {
    single_cross: Box<dyn Cross<G>>,
}

/// Child chromosome is arithmetic mean of parent chromosomes. Result of cross is single child.
/// The chromosomes must be numeric type.
pub struct CrossMean;

/// For float type chromosomes (f32, f64). Child chromosome is geometric mean of parent
/// chromosomes. Result of cross is single child.
pub struct FloatCrossGeometricMean;

/// Bitwise cross. Use single point crossing. Result of cross is single child.
pub struct CrossBitwise {
    random: ThreadRng,
}

/// Bitwise cross for float type chromosomes. Exponent and mantissa will be crossed independently.
/// Use single point crossing. The sign is taken from one of parents at random.
pub struct FloatCrossExp {
    random: ThreadRng,
}

impl CrossMean {
    pub fn new() -> Self {
        Self {}
    }
}

/// ```
/// use optlib::genetic::cross;
/// use optlib::genetic::Cross;
///
/// assert!(cross::CrossMean::new().cross(&vec![&1.0_f32, &2.0_f32]) == vec![1.5_f32]);
/// assert!(cross::CrossMean::new().cross(&vec![&0.0_f64, &1.0_f64]) == vec![0.5_f64]);
/// assert!(cross::CrossMean::new().cross(&vec![&0.0_f64, &1.0_f64, &2.0_f64]) == vec![1.0_f64]);
/// ```
impl<G: NumCast + Num + Clone> Cross<G> for CrossMean {
    fn cross(&mut self, parents_genes: &[&G]) -> Vec<G> {
        assert!(parents_genes.len() >= 2);
        let mut result: G = parents_genes
            .iter()
            .fold(G::zero(), |x, y| x + (**y).clone());

        result = result / G::from(parents_genes.len()).unwrap();
        vec![result]
    }
}

impl FloatCrossGeometricMean {
    /// Constructor.
    pub fn new() -> Self {
        Self {}
    }
}

/// ```
/// use optlib::genetic::cross;
/// use optlib::genetic::Cross;
///
/// let mut crosser = cross::FloatCrossGeometricMean::new();
///
/// let parents_genes_equal = vec![&1.0_f32, &1.0_f32];
/// assert!(crosser.cross(&parents_genes_equal).len() == 1);
/// assert!(crosser.cross(&parents_genes_equal)[0] - 1.0_f32 < 1e-7);
///
/// let parents_genes_1 = vec![&1.0_f32, &2.0_f32];
/// assert!(crosser.cross(&parents_genes_1).len() == 1);
/// assert!(crosser.cross(&parents_genes_1)[0] - 1.41421 < 1e-4);
///
/// let parents_genes_2 = vec![&1.0_f64, &2.0_f64, &3.0_f64];
/// assert!(crosser.cross(&parents_genes_2).len() == 1);
/// assert!(crosser.cross(&parents_genes_2)[0] - 1.81712 < 1e-4);
/// ```
impl<G: Float> Cross<G> for FloatCrossGeometricMean {
    fn cross(&mut self, parents_genes: &[&G]) -> Vec<G> {
        assert!(parents_genes.len() >= 2);
        let mut result: G = parents_genes.iter().fold(G::one(), |x, y| x * (**y));

        result = result.powf(G::one() / G::from(parents_genes.len()).unwrap());
        vec![result]
    }
}

impl CrossBitwise {
    /// Constructor.
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl Cross<f64> for CrossBitwise {
    fn cross(&mut self, parents_genes: &[&f64]) -> Vec<f64> {
        assert_eq!(parents_genes.len(), 2);
        let size = mem::size_of::<f64>() * 8;
        let between = Uniform::new(1, size);
        let pos = between.sample(&mut self.random);

        vec![cross_f64(*parents_genes[0], *parents_genes[1], pos)]
    }
}

impl Cross<f32> for CrossBitwise {
    fn cross(&mut self, parents_genes: &[&f32]) -> Vec<f32> {
        assert_eq!(parents_genes.len(), 2);
        let size = mem::size_of::<f32>() * 8;
        let between = Uniform::new(1, size);
        let pos = between.sample(&mut self.random);

        vec![cross_f32(*parents_genes[0], *parents_genes[1], pos)]
    }
}

impl<G> VecCrossAllGenes<G> {
    pub fn new(single_cross: Box<dyn Cross<G>>) -> Self {
        Self { single_cross }
    }
}

impl<G> Cross<Vec<G>> for VecCrossAllGenes<G> {
    fn cross(&mut self, parents: &[&Vec<G>]) -> Vec<Vec<G>> {
        assert!(parents.len() == 2);

        let parent_1 = parents[0];
        let parent_2 = parents[1];

        let gene_count = parent_1.len();
        let mut child = vec![];

        for n in 0..gene_count {
            let mut new_gene = self
                .single_cross
                .cross(vec![&parent_1[n], &parent_2[n]].as_slice());
            child.append(&mut new_gene);
        }
        vec![child]
    }
}

impl FloatCrossExp {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl<T: Float> Cross<T> for FloatCrossExp {
    fn cross(&mut self, parents_genes: &[&T]) -> Vec<T> {
        assert_eq!(parents_genes.len(), 2);
        // mantissa: u64, exponent: i16, sign: i8
        let (mantissa_1, exponent_1, sign_1) = parents_genes[0].integer_decode();
        let (mantissa_2, exponent_2, sign_2) = parents_genes[1].integer_decode();

        let mantissa_size = mem::size_of_val(&mantissa_1) * 8;
        let exponent_size = mem::size_of_val(&exponent_1) * 8;

        let mantissa_between = Uniform::new(1, mantissa_size);
        let exponent_between = Uniform::new(1, exponent_size);

        let mantissa_pos = mantissa_between.sample(&mut self.random);
        let exponent_pos = exponent_between.sample(&mut self.random);

        let mantissa_child = cross_u64(mantissa_1, mantissa_2, mantissa_pos);
        let exponent_child = cross_i16(exponent_1, exponent_2, exponent_pos);

        let sign_child = match Uniform::new_inclusive(0i8, 1i8).sample(&mut self.random) {
            0 => sign_1,
            1 => sign_2,
            _ => panic!("Invalid random value in FloatCrossExp"),
        };

        vec![
            T::from(sign_child).unwrap()
                * T::from(mantissa_child).unwrap()
                * T::from(exponent_child).unwrap().exp2(),
        ]
    }
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_u64(0u64, std::u64::MAX, 1), 1u64);
/// assert_eq!(cross::cross_u64(0u64, std::u64::MAX, 4), 0b_1111_u64);
/// assert_eq!(cross::cross_u64(0u64, std::u64::MAX, 63),
/// 0b_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_u64);
/// assert_eq!(cross::cross_u64(std::u64::MAX, 0u64, 4),
/// 0b_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_u64);
/// ```
pub fn cross_u64(parent_1: u64, parent_2: u64, pos: usize) -> u64 {
    let size = mem::size_of::<f64>() * 8;
    let mask_parent_1 = !0u64 << pos;
    let mask_parent_2 = !0u64 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_u32(0u32, std::u32::MAX, 1), 1u32);
/// assert_eq!(cross::cross_u32(0u32, std::u32::MAX, 4), 0b_1111_u32);
/// assert_eq!(cross::cross_u32(0u32, std::u32::MAX, 31), 0b_0111_1111_1111_1111_1111_1111_1111_1111_u32);
/// assert_eq!(cross::cross_u32(std::u32::MAX, 0u32, 4), 0b_1111_1111_1111_1111_1111_1111_1111_0000_u32);
/// ```
pub fn cross_u32(parent_1: u32, parent_2: u32, pos: usize) -> u32 {
    let size = mem::size_of::<u32>() * 8;
    let mask_parent_1 = !0u32 << pos;
    let mask_parent_2 = !0u32 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
///# Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// // -1i16 == 0b_1111_1111_1111_1111
/// assert_eq!(cross::cross_i16(0i16, -1i16, 4), 0b_0000_0000_0000_1111_i16);
/// assert_eq!(cross::cross_i16(0i16, -1i16, 1), 0b_0000_0000_0000_0001_i16);
/// assert_eq!(cross::cross_i16(0i16, -1i16, 15), 0b_0111_1111_1111_1111_i16);
/// assert_eq!(cross::cross_i16(0i16, -1i16, 8), 0b_0000_0000_1111_1111_i16);
///
/// // -1i16 == 0b_1111_1111_1111_1111
/// // -16i16 == 0b_1111_1111_1111_0000
/// assert_eq!(cross::cross_i16(-1i16, 0i16, 4), -16i16);
///
/// // -1i16 == 0b_1111_1111_1111_1111
/// // -32768i16 == 0b_1000_0000_0000_0000
/// assert_eq!(cross::cross_i16(-1i16, 0i16, 15), -32768i16);
/// ```
pub fn cross_i16(parent_1: i16, parent_2: i16, pos: usize) -> i16 {
    let size = mem::size_of::<i16>() * 8;
    let mask_parent_1 = !0i16 << pos;
    let mask_parent_2 = std::i16::MAX >> (size - pos - 1);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_u16(0b_0000_0000_0000_0000, 0b_1111_1111_1111_1111, 4), 0b_1111);
/// assert_eq!(cross::cross_u16(0b_0000_0000_0000_0000, 0b_1111_1111_1111_1111, 1), 0b_1);
/// assert_eq!(cross::cross_u16(0b_0000_0000_0000_0000, 0b_1111_1111_1111_1111, 8), 0b_0000_0000_1111_1111);
/// assert_eq!(cross::cross_u16(0b_0000_0000_0000_0000, 0b_1111_1111_1111_1111, 15), 0b_0111_1111_1111_1111);
/// ```
pub fn cross_u16(parent_1: u16, parent_2: u16, pos: usize) -> u16 {
    let size = mem::size_of::<u16>() * 8;
    let mask_parent_1 = !0u16 << pos;
    let mask_parent_2 = !0u16 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
///# Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_i8(0b_0000_0000_i8, 0b_0111_1111_i8, 4), 0b_0000_1111_i8);
/// assert_eq!(cross::cross_i8(0b_0000_0000_i8, 0b_0111_1111_i8, 1), 0b_0000_0001_i8);
/// assert_eq!(cross::cross_i8(0b_0000_0000_i8, 0b_0111_1111_i8, 7), 0b_0111_1111_i8);
/// assert_eq!(cross::cross_i8(0b_0000_0000_i8, 0b_0111_1111_i8, 6), 0b_0011_1111_i8);
///
/// // -1i8 == 0b_1111_1111
/// // -16i8 == 0b_1111_0000
/// assert_eq!(cross::cross_i8(-1i8, 0i8, 4), -16i8);
///
/// // -1i8 == 0b_1111_1111
/// // -128i8 == 0b_1000_0000
/// assert_eq!(cross::cross_i8(-1i8, 0i8, 7), -128i8);
/// ```
pub fn cross_i8(parent_1: i8, parent_2: i8, pos: usize) -> i8 {
    let size = mem::size_of::<i8>() * 8;
    let mask_parent_1 = !0i8 << pos;
    let mask_parent_2 = std::i8::MAX >> (size - pos - 1);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_u8(0b_0000_0000, 0b_1111_1111, 4), 0b_0000_1111);
/// assert_eq!(cross::cross_u8(0b_0000_0000, 0b_1111_1111, 1), 0b_0000_0001);
/// assert_eq!(cross::cross_u8(0b_0000_0000, 0b_1111_1111, 7), 0b_0111_1111);
/// ```
pub fn cross_u8(parent_1: u8, parent_2: u8, pos: usize) -> u8 {
    let size = mem::size_of::<u8>() * 8;
    let mask_parent_1 = !0u8 << pos;
    let mask_parent_2 = !0u8 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_f32(0f32, f32::from_bits(std::u32::MAX), 1), f32::from_bits(0b_0001));
/// assert_eq!(cross::cross_f32(0f32, f32::from_bits(std::u32::MAX), 4), f32::from_bits(0b_1111));
/// assert_eq!(cross::cross_f32(0f32, f32::from_bits(std::u32::MAX), 30),
///                             f32::from_bits(0b_0011_1111_1111_1111_1111_1111_1111_1111_u32));
/// ```
pub fn cross_f32(parent_1: f32, parent_2: f32, pos: usize) -> f32 {
    let parent_1_bits = parent_1.to_bits();
    let parent_2_bits = parent_2.to_bits();

    let child_bits = cross_u32(parent_1_bits, parent_2_bits, pos);
    f32::from_bits(child_bits)
}

/// Single point crossing.
///
/// # Parameters
/// * `parent_1`, `parent_2` - parents for crossing.
/// * `pos` - position for bytes exchange. The position is counted from right.
///
/// Returns single child.
///
/// # Examples
///
/// ```
/// use optlib::genetic::cross;
///
/// assert_eq!(cross::cross_f64(0f64, f64::from_bits(std::u64::MAX), 1), f64::from_bits(0b_0001));
/// assert_eq!(cross::cross_f64(0f64, f64::from_bits(std::u64::MAX), 4), f64::from_bits(0b_1111));
/// assert_eq!(cross::cross_f64(0f64, f64::from_bits(std::u64::MAX), 62),
///                             f64::from_bits(0b_0011_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_u64));
/// ```
pub fn cross_f64(parent_1: f64, parent_2: f64, pos: usize) -> f64 {
    let parent_1_bits = parent_1.to_bits();
    let parent_2_bits = parent_2.to_bits();

    let child_bits = cross_u64(parent_1_bits, parent_2_bits, pos);
    f64::from_bits(child_bits)
}
