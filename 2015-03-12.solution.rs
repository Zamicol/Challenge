// Rust solution via @jrick

use std::cmp;
use std::num::{Int, NumCast};

fn percents<T: Int>(input: &Vec<T>) -> Vec<f64> {
    let (min, max) = input.iter().fold((T::max_value(), T::min_value()),
        |m, &e| (cmp::min(m.0, e), cmp::max(m.1, e))
    );
    let diff = f64::from(max - min).unwrap();
    input.iter().map(|elem| f64::from(*elem - min).unwrap() / diff).collect()
}

fn main() {
    let input = vec![99334, 100, 200, 300, 99991];
    let out = percents(&input);
    println!("{:?}", out);
}
