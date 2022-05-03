// -*- coding:utf-8-unix -*-

use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let n = n as u64;
    let max = (n as f64).sqrt() as u64;
    let mut min = std::u64::MAX;

    for i in 1..=max {
        if n % i == 0 {
            let a = i;
            let b = (n as u64) / i;

            min = min.min(a.max(b));
        }
    }

    println!("{}", min.to_string().len());
}
