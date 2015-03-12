# Rust solution via @jrick

use std::num::{Int, NumCast};

fn percents<T: Int + NumCast>(input: &Vec<T>) -> Vec<f64> {
    let max: T = {
        let mut max: T = T::zero();
        for elem in input {
            if *elem > max {
                max = *elem;
            }
        }
        max
    };
    input.iter().map(|elem|
        100f64 * f64::from::<T>(*elem).unwrap() / f64::from::<T>(max).unwrap()
    ).collect()
}

fn main() {
    let input = vec![99334, 1, 2, 3, 4, 5, 6, 7, 8, 9, 99991];
    let out = percents(&input);
    println!("{:?}", out);
}
