// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        p: (u32, u32, u32),
    }

    let (a, b, k) = (p.0 as u32, p.1 as u32, p.2 as usize);
    let max = a.max(b);

    let mut divisors = (1..=max).map(|i| {
        if a % i ==0 && b % i == 0 {
            return i;
        } else {
            return 0;
        }
    }).collect::<Vec<u32>>();

    divisors.sort();


    println!("{}", divisors[&divisors.len() - k]);
}
