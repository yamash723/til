// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut count = 0;
    for a_i in 0..=a as usize {
        for b_i in 0..=b as usize {
            for c_i in 0..=c as usize {
                if a_i * 500 + b_i * 100 + c_i * 50 == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
