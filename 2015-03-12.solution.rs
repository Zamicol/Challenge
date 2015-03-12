# Rust solution via @jrick

use std::cmp;
use std::num::{Int, NumCast};

fn percents<T: Int>(input: &Vec<T>) -> Vec<f64> {
    let mul = f64::from(input.iter().fold(T::zero(), |m, &e| cmp::max(m, e))).unwrap();
    input.iter().map(|elem| mul * f64::from(*elem).unwrap()).collect()
}

fn main() {
    let input = vec![99334, 1, 2, 3, 4, 5, 6, 7, 8, 9, 99991];
    let out = percents(&input);
    println!("{:?}", out);
}
